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
    // Overall status of the batch.
    pub batch: BatchStatus,
    // A status for every command in the batch.
    pub commands: Vec<CommandStatus>,
}

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum BatchStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "done")]
    Done {
        succeeded: bool
    }
}

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum CommandStatus {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "done")]
    Done {
        succeeded: bool
    }
}

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("describe_commands called"))
}