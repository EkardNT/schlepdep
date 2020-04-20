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
    DispatchCommands
}

impl Operation {
    fn all() -> &'static [Self] {
        &[
            Self::ReceiveCommands,
            Self::DispatchCommands
        ]
    }

    fn path_regex(&self) -> &'static str {
        match self {
            Self::ReceiveCommands => "/api/dispatch/receive_commands",
            Self::DispatchCommands => "/api/dispatch/dispatch_commands",
        }
    }

    fn method(&self) -> &'static Method {
        match self {
            Self::ReceiveCommands => &Method::GET,
            Self::DispatchCommands => &Method::POST,
        }
    }

    async fn invoke(&self, req: Request<Body>, path_regex: &Regex) -> Response<Body> {
        match self {
            Self::ReceiveCommands => receive_commands(req, path_regex).await,
            Self::DispatchCommands => dispatch_commands(req, path_regex).await,
        }
    }
}

async fn receive_commands(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("receive_commands called"))
}

async fn dispatch_commands(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("dispatch_commands called"))
}