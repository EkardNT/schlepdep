use std::collections::HashMap;

use hyper::{Body, Method, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    // Command batch id.
    pub batch_id: String
}

#[derive(Serialize)]
pub struct Output {
}

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("delete_commands called"))
}