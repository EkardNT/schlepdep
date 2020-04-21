use hyper::{Body, Method, Request, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Input {
    // Command batch id.
    pub batch_id: String,
    // Index of the command within the batch that is being started.
    pub command_index: usize,
    // Randomly generated retry nonce. If the client retries, then each retry should have
    // the same nonce, to allow for idempotency.
    pub nonce: String
}

#[derive(Serialize)]
#[serde(tag = "instruction")]
pub enum Output {
    // The client should not execute the command and should discard the entire command
    // batch.
    #[serde(rename = "discard")]
    Discard,
    // The client should continue with executing the command.
    #[serde(rename = "continue")]
    Continue {
        // Initial attempt token.
        attempt_token: String
    }
}

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("start_command called"))
}