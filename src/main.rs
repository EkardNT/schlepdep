mod operations;

use std::net::{Ipv4Addr, SocketAddr};
use std::rc::Rc;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::operations::Router;

use core_affinity::CoreId;
use crossbeam::channel::{self, Sender, Receiver, TryRecvError, TrySendError};
use hyper::{Body, Request, Response, Version};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use net2::TcpBuilder;
use net2::unix::UnixTcpBuilderExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio::task::{spawn_local, yield_now, LocalSet};
use tokio::time::delay_for;

const MAX_CONNS_PER_CORE: usize = 65536;
const MAX_ACCEPT_QUEUE_PER_CORE: usize = 64;

struct AcceptedConn {
    stream: TcpStream,
    remote_addr: SocketAddr
}

fn main() {
    let core_ids = core_affinity::get_core_ids()
        .expect("Failed to get core ids");

    let max_conns = core_ids.len() * MAX_CONNS_PER_CORE;
    let max_conns_semaphore = Arc::new(Semaphore::new(max_conns));

    let accept_queue_max = core_ids.len() * MAX_ACCEPT_QUEUE_PER_CORE;
    let accept_queue_semaphore = Arc::new(Semaphore::new(accept_queue_max));
    let (accept_queue_tx, accept_queue_rx) = channel::bounded(accept_queue_max);

    let worker_handles = start_worker_threads(&core_ids, accept_queue_rx, accept_queue_semaphore.clone());
    let acceptor_handles = start_acceptor_threads(&core_ids, accept_queue_tx, max_conns_semaphore, accept_queue_semaphore);
    for handle in worker_handles {
        handle.join().expect("Worker thread panicked");
    }
    for handle in acceptor_handles {
        handle.join().expect("Acceptor thread panicked");
    }
}

fn start_worker_threads(
        core_ids: &[CoreId],
        accept_queue: Receiver<AcceptedConn>,
        accept_queue_semaphore: Arc<Semaphore>) -> Vec<JoinHandle<()>> {
    core_ids.iter().cloned().enumerate().map(|(thread_index, core_id)| {
        let accept_queue = accept_queue.clone();
        let accept_queue_semaphore = accept_queue_semaphore.clone();
        let thread_name = format!("dispatch-worker-{}", thread_index);
        std::thread::Builder::new()
            .name(thread_name)
            .stack_size(10 * 1014 * 1024)
            .spawn(move || {
                core_affinity::set_for_current(core_id);
                worker_main(accept_queue, accept_queue_semaphore);
            })
            .expect("Failed to spawn worker thread")
    }).collect()
}

fn start_acceptor_threads(
        core_ids: &[CoreId],
        accept_queue: Sender<AcceptedConn>,
        max_conns_semaphore: Arc<Semaphore>,
        accept_queue_semaphore: Arc<Semaphore>) -> Vec<JoinHandle<()>> {
    core_ids.iter().cloned().enumerate().map(|(thread_index, core_id)| {
        let accept_queue = accept_queue.clone();
        let max_conns_semaphore = max_conns_semaphore.clone();
        let accept_queue_semaphore = accept_queue_semaphore.clone();
        let thread_name = format!("dispatch-acceptor-{}", thread_index);
        std::thread::Builder::new()
            .name(thread_name)
            .stack_size(10 * 1024)
            .spawn(move || {
                core_affinity::set_for_current(core_id);
                acceptor_main(accept_queue, max_conns_semaphore, accept_queue_semaphore);
            })
            .expect("Failed to spawn acceptor thread")
    }).collect()
}

fn worker_main(
        accept_queue: Receiver<AcceptedConn>,
        accept_queue_semaphore: Arc<Semaphore>) {
    println!("Hello world! I'm a worker thread.");
    let mut rt = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime on worker thread");
    let local = LocalSet::new();
    // Spawn a future which continuously reads from the accept_queue.
    local.spawn_local(async move {
        let router = Rc::new(Router::new());
        // TODO: consider making these thresholds time-based rather than count-based.
        let mut consecutive_empties = 0usize;
        let mut consecutive_present = 0usize;
        loop {
            match accept_queue.try_recv() {
                Ok(conn) => {
                    // Allow more connections to be accept()ed.
                    accept_queue_semaphore.add_permits(1);
                    // Handle the request in a separate task.
                    spawn_local(handle_conn(conn, router.clone()));
                    consecutive_empties = 0;
                    consecutive_present = consecutive_present.wrapping_add(1);
                    // If we get a bunch of new connections all at once, make sure to yield
                    // occasionally to allow response-generating futures to execute.
                    if consecutive_present % 16 == 0 {
                        yield_now().await;
                    }
                },
                Err(TryRecvError::Empty) => {
                    consecutive_empties = consecutive_empties.wrapping_add(1);
                    consecutive_present = 0;
                    // Due to lack of async-enabled mpmc channel, we spin in a try_recv()
                    // loop, yielding every iteration to allow other futures to execute.
                    // However this will result in a CPU busyloop. If we see a sustained
                    // number of empty queue results then we will put the task to sleep
                    // to reduce CPU usage.
                    if consecutive_empties % 64 == 0 {
                        // This delay is a tradeoff between additional latency on requests
                        // and CPU usage.
                        delay_for(Duration::from_millis(100)).await;
                    } else {
                        yield_now().await;
                    }
                },
                Err(TryRecvError::Disconnected) => {
                    // Shut down.
                    break;
                }
            };
        }
    });
    // Block until the LocalSet is complete, aka both the accept_queue reader task and the
    // spawned per-connection tasks are complete.
    rt.block_on(local);
}

fn acceptor_main(
        accept_queue: Sender<AcceptedConn>,
        max_conns_semaphore: Arc<Semaphore>,
        accept_queue_semaphore: Arc<Semaphore>) {
    println!("Hello world! I'm an acceptor thread.");
    let mut rt = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime on acceptor thread");
    let local = tokio::task::LocalSet::new();
    let listener = TcpBuilder::new_v4()
        .expect("Failed to create TcpBuilder")
        .reuse_address(true)
        .expect("Failed to set reuse_address(true)")
        .reuse_port(true)
        .expect("Failed to set reuse_port(true)")
        .bind((Ipv4Addr::new(127, 0, 0, 1), 43316))
        .expect("Failed to bind socket")
        .listen(128)
        .expect("Failed to begin listening on socket");
    // Continuously accept connections and sends them to the accept_queue.
    rt.block_on(async move {
        let mut listener = TcpListener::from_std(listener)
            .expect("Failed to convert std TcpListener to tokio");

        loop {
            // Do not immediately forget() this permit so that it can be released in case of
            // accept error.
            let permit = accept_queue_semaphore.acquire().await;

            match listener.accept().await {
                Ok((stream, remote_addr)) => {
                    // The two error cases are Full and Disconnected. Full should not happen
                    // because the accept_queue_semaphore is the same size as the accept_queue,
                    // so getting a permit should mean that the accept_queue has room.
                    // Disconnected should not happen because it is acceptors which listen
                    // for shutdown and drop their senders first.
                    accept_queue.try_send(AcceptedConn { stream, remote_addr })
                        .expect("Sending the new conn from the acceptor to the worker \
                            queue failed unexpectedly");
                    // The permit is added back by the worker thread which dequeues the connection.
                    permit.forget();
                },
                Err(_err) => {
                    // TODO: warn log
                    // Release before waiting.
                    std::mem::drop(permit);
                    delay_for(Duration::from_millis(15)).await;
                }
            }
        }
    });
}

async fn handle_conn(conn: AcceptedConn, router: Rc<Router>) {
    let service = service_fn(|req: Request<Body>| {
        // This function may be invoked multiple times for pipelined requests on the same
        // connection, so we need to clone things for each invocation.
        let router = router.clone();
        async move {
            // This error type never occurs. Wish that ! worked.
            Result::<_, Box<dyn std::error::Error + Send + Sync + 'static>>::Ok(handle_request(req, router).await)
        }
    });
    let mut http = Http::new().with_executor(LocalExec);
    http.http1_only(true);
    if let Err(err) = http.serve_connection(conn.stream, service).await {
        // TODO: warn log
    }
}

async fn handle_request(req: Request<Body>, router: Rc<Router>) -> Response<Body> {
    router.route(req).await.unwrap_or_else(|| Response::builder()
        .status(404)
        .body(Body::from("No matching operation"))
        .expect("Failed to build 404 response for unroutable request"))
}

// Copied from https://github.com/hyperium/hyper/blob/master/examples/single_threaded.rs
#[derive(Clone, Copy, Debug)]
struct LocalExec;

impl<F> hyper::rt::Executor<F> for LocalExec
where
    F: std::future::Future + 'static, // not requiring `Send`
{
    fn execute(&self, fut: F) {
        // This will spawn into the currently running `LocalSet`.
        spawn_local(fut);
    }
}