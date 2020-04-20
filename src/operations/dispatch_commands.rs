use crate::database::Database;
use crate::operations::run_operation;

use std::collections::HashMap;
use std::sync::Arc;

use hyper::{Body, Method, Request, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub async fn handle(req: Request<Body>, database: Arc<Database>) -> Response<Body> {
    run_operation(req, database, 32 * 1024, |req: Request<Input>, database| async move {
        let batch_id = format!("{}", Uuid::new_v4().to_hyphenated());

        // TODO:
        // - Generate batch UUID.
        // - Create idempotency record.
        //      - Record's key should be computed based on a hash of the api name and all parameters.
        //      - If already exists, replace the generated batch UUID with the one from the existing idempotency record.
        // - Create command definition records
        //      - If already exists, do nothing
        // - Create command batch record
        //      - If already exists, do nothing (do not overwrite!)
        // - Check for remote poll against input target_name. If found, interrupt.
        // - Return batch id.
        Response::new(Output { batch_id })
    }).await
}