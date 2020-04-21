use hyper::{Body, Method, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    // Command batch id.
    pub batch_id: String,
    // Attempt token.
    pub attempt_token: String
}

#[derive(Serialize)]
#[serde(tag = "instruction")]
pub enum Output {
    // The client should cancel the running command (if possible) and should discard the
    // entire command batch.
    #[serde(rename = "discard")]
    Discard,
    // The client should continue with executing the command.
    #[serde(rename = "continue")]
    Continue
}

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("heartbeat_command called"))
}