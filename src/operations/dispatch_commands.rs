use std::collections::HashMap;

use hyper::{Body, Method, Request, Response};
use regex::{Captures, Regex, RegexSet};
use serde::{Deserialize, Serialize};

// TODO: dispatcher permissions (or just infer from caller's auth mechanisms?).
#[derive(Deserialize)]
pub struct Input {
    // The target the commands are being dispatched against.
    pub target_name: String,
    // All the commands in the batch, in the order in which they must be executed.
    pub commands: Vec<Command>,
    // Randomly generated retry nonce. If the client retries, then each retry should have
    // the same nonce, to allow for idempotency.
    pub nonce: String,
    // Channel on which notifications will be sent when the batch is complete.
    pub batch_complete_notification: Option<Channel>,
}

// TODO: executor permissions
#[derive(Deserialize)]
pub struct Command {
    // Command name.
    pub name: String,
    // Small per-command data that will be passed down to the executor.
    pub data: String,
    // Maximum retries for this command. The actual max attempts is max_retries + 1.
    pub max_retries: usize,
    // Channel on which notifications will be sent when the command becomes available.
    pub command_available_notification: Option<Channel>,
    // Channel on which notifications will be sent when the executor makes progress on a
    // command (starts an attempt, completes an attempt, etc).
    pub command_progress_notification: Option<Channel>,
    // If true, then if all retries are exhausted due to failure the batch will fail. If
    // false, then retries will still be used but if the retries are exhausted then the
    // batch will proceed to the next command.
    pub success_required: bool,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Channel {
    #[serde(rename = "http")]
    HTTP {
        endpoint: String,
        additional_headers: Option<HashMap<String, String>>,
    },
    #[serde(rename = "aws_sqs")]
    SQS {
        queue_url: String,
    },
    #[serde(rename = "aws_sns")]
    SNS {
        target_arn: String,
    }
}

#[derive(Serialize)]
pub struct Output {
    // The new command batch id.
    pub batch_id: String
}

pub async fn handle(_req: Request<Body>, _path_regex: &Regex) -> Response<Body> {
    Response::new(Body::from("dispatch_commands called"))
}