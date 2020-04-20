use std::collections::HashMap;

use hyper::{Body, Method, Request, Response};
use regex::{Captures, Regex, RegexSet};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    // Command batch id.
    pub batch_id: String,
    // 0-based command index to get details of.
    pub command_index: usize,
}

#[derive(Serialize)]
pub struct Output {
    // Overall status of the command.
    pub command: CommandStatus,
    // A status for every attempt in the command.
    pub attempts: Vec<AttemptStatus>,
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

#[derive(Serialize)]
#[serde(tag = "status")]
pub enum AttemptStatus {
    #[serde(rename = "available")]
    Available {
        available_epoch_millis: usize,
    },
    #[serde(rename = "started")]
    Started {
        heartbeats: usize,
        available_epoch_millis: usize,
        start_epoch_millis: usize,
    },
    #[serde(rename = "done")]
    Done {
        data: String,
        succeeded: bool,
        heartbeats: usize,
        available_epoch_millis: usize,
        start_epoch_millis: usize,
        complete_epoch_millis: usize,
    }
}

pub async fn handle(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("describe_command called"))
}