mod complete_command;
mod delete_commands;
mod describe_command;
mod describe_commands;
mod dispatch_commands;
mod heartbeat_command;
mod receive_commands;
mod start_command;

use crate::errors::{
    body_read_failed,
    body_too_large,
    req_json_parse,
    internal,
    no_content_length,
};
use crate::database::Database;

use std::future::Future;
use std::sync::Arc;

use hyper::{Body, Method, Request, Response};
use regex::{Captures, Regex, RegexSet};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Router {
    path_set: RegexSet,
    all_operations: Vec<Operation>
}

impl Router {
    pub fn new() -> Self {
        let path_set = RegexSet::new(Operation::all().iter()
            .map(|op| op.path_regex()))
            .expect("One of the operation regexes was invalid");
        let all_operations = Operation::all().iter().cloned().collect();
        Self {
            path_set,
            all_operations
        }
    }

    pub async fn route(&self, req: Request<Body>, database: Arc<Database>) -> Option<Response<Body>> {
        // So few operations that doing anything more complicated is pointless.
        for op_index in self.path_set.matches(req.uri().path()) {
            if let Some(op) = self.all_operations.get(op_index) {
                if req.method() == op.method() {
                    return Some(op.invoke(req, database).await);
                }
            }
        }
        None
    }
}

#[derive(Clone)]
enum Operation {
    ReceiveCommands,
    DispatchCommands,
    StartCommand,
    HeartbeatCommand,
    CompleteCommand,
    DescribeCommands,
    DescribeCommand,
    DeleteCommands,
}

impl Operation {
    fn all() -> &'static [Self] {
        &[
            Self::ReceiveCommands,
            Self::DispatchCommands,
            Self::StartCommand,
            Self::HeartbeatCommand,
            Self::CompleteCommand,
            Self::DescribeCommands,
            Self::DescribeCommand,
            Self::DeleteCommands
        ]
    }

    fn path_regex(&self) -> &'static str {
        match self {
            Self::ReceiveCommands => "/api/dispatch/receive_commands",
            Self::DispatchCommands => "/api/dispatch/dispatch_commands",
            Self::StartCommand => "/api/dispatch/start_command",
            Self::HeartbeatCommand => "/api/dispatch/heartbeat_command",
            Self::CompleteCommand => "/api/dispatch/complete_command",
            Self::DescribeCommands => "/api/dispatch/describe_commands",
            Self::DescribeCommand => "/api/disptch/describe_command",
            Self::DeleteCommands => "/api/dispatch/delete_commands",
        }
    }

    fn method(&self) -> &'static Method {
        &Method::POST
    }

    async fn invoke(&self, req: Request<Body>, database: Arc<Database>) -> Response<Body> {
        match self {
            Self::ReceiveCommands => receive_commands::handle(req).await,
            Self::DispatchCommands => dispatch_commands::handle(req, database).await,
            Self::StartCommand => start_command::handle(req).await,
            Self::HeartbeatCommand => heartbeat_command::handle(req).await,
            Self::CompleteCommand => complete_command::handle(req).await,
            Self::DescribeCommands => describe_commands::handle(req).await,
            Self::DescribeCommand => describe_command::handle(req).await,
            Self::DeleteCommands => delete_commands::handle(req).await,
        }
    }
}

pub async fn run_operation<Op, In, Out, Fut>(
        req: Request<Body>,
        database: Arc<Database>,
        max_body_size: usize,
        op: Op) -> Response<Body>
where
    Op: FnOnce(Request<In>, Arc<Database>) -> Fut,
    In: for<'a> Deserialize<'a>,
    Out: Serialize,
    Fut: Future<Output = Response<Out>>
{
    let content_length = match req.headers().get("Content-Length")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.parse::<usize>().ok()) {
        Some(content_length) => content_length,
        None => {
            // TODO: debug log
            return no_content_length();
        }
    };
    if content_length > max_body_size {
        // TODO: debug log
        return body_too_large();
    }
    let (parts, in_body) = req.into_parts();
    // TODO: custom version of to_bytes which stops as soon as the max_body_size is exceeded.
    let bytes = match hyper::body::to_bytes(in_body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            // TODO: debug log
            return body_read_failed();
        }
    };
    if bytes.len() > max_body_size {
        // TODO: debug log
        return body_too_large();
    }
    let input: In = match serde_json::from_slice(&bytes) {
        Ok(input) => input,
        Err(err) => {
            // TODO: debug log
            return req_json_parse();
        }
    };
    let req = Request::from_parts(parts, input);
    let (parts, output) = op(req, database).await.into_parts();
    let out_bytes = match serde_json::to_vec(&output) {
        Ok(out_bytes) => out_bytes,
        Err(err) => {
            // TODO: warn log
            return internal();
        }
    };
    let out_body = Body::from(out_bytes);
    return Response::from_parts(parts, out_body);
}