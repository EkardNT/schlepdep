use hyper::{Body, Method, Request, Response};
use regex::{Captures, Regex, RegexSet};

#[derive(Clone)]
pub struct Router {
    path_set: RegexSet,
    all_operations: Vec<(Operation, Regex)>
}

impl Router {
    pub fn new() -> Self {
        let path_set = RegexSet::new(Operation::all().iter()
            .map(|op| op.path_regex()))
            .expect("One of the operation regexes was invalid");
        let all_operations = Operation::all().iter()
            .map(|op| (*op, Regex::new(op.path_regex()).expect("Invalid regex")))
            .collect();
        Self {
            path_set,
            all_operations
        }
    }

    pub async fn route(&self, req: Request<Body>) -> Option<Response<Body>> {
        // So few operations that doing anything more complicated is pointless.
        for op_index in self.path_set.matches(req.uri().path()) {
            if let Some((op, regex)) = self.all_operations.get(op_index) {
                if req.method() == op.method() {
                    // Pass down a reference to the regex so that handlers can inspect
                    // capture groups if necessary (to extract parameters).
                    return Some(op.invoke(req, &regex).await);
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone)]
enum Operation {
    ReceiveCommands,
    DispatchCommands,
    AcknowledgeCommand,
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
            Self::AcknowledgeCommand,
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
            Self::AcknowledgeCommand => "/api/dispatch/acknowledge_command",
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

    async fn invoke(&self, req: Request<Body>, path_regex: &Regex) -> Response<Body> {
        match self {
            Self::ReceiveCommands => receive_commands(req, path_regex).await,
            Self::DispatchCommands => dispatch_commands(req, path_regex).await,
            Self::AcknowledgeCommand => acknowledge_command(req, path_regex).await,
            Self::HeartbeatCommand => heartbeat_command(req, path_regex).await,
            Self::CompleteCommand => complete_command(req, path_regex).await,
            Self::DescribeCommands => describe_commands(req, path_regex).await,
            Self::DescribeCommand => describe_command(req, path_regex).await,
            Self::DeleteCommands => delete_commands(req, path_regex).await,
        }
    }
}

async fn receive_commands(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("receive_commands called"))
}

async fn dispatch_commands(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("dispatch_commands called"))
}

async fn acknowledge_command(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("acknowledge_command called"))
}

async fn heartbeat_command(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("heartbeat_command called"))
}

async fn complete_command(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("complete_command called"))
}

async fn describe_commands(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("describe_commands called"))
}

async fn describe_command(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("describe_command called"))
}

async fn delete_commands(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("delete_commands called"))
}
