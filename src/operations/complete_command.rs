use hyper::{Body, Method, Request, Response};
use regex::{Captures, Regex, RegexSet};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    // Command batch id.
    pub batch_id: String,
    // Attempt token.
    pub attempt_token: String,
    // True if the command succeeded, false if it failed.
    pub success: bool,
    // Command completion data. May be provided for both successful and failed commands.
    pub data: Option<String>
}

#[derive(Serialize)]
#[serde(tag = "instruction")]
pub enum Output {
    // The client should discard the entire command batch.
    Discard,
    // The client should proceed to the next command. If no more commands, then discard the
    // command batch.
    NextCommand,
    // The client should retry the same command again.
    SameCommand,
}

pub async fn handle(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("complete_command called"))
}