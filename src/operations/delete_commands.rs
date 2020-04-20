use std::collections::HashMap;

use hyper::{Body, Method, Request, Response};
use regex::{Captures, Regex, RegexSet};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    // Command batch id.
    pub batch_id: String
}

#[derive(Serialize)]
pub struct Output {
}

pub async fn handle(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("delete_commands called"))
}