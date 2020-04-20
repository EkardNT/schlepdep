use hyper::{Body, Method, Request, Response};
use serde::{Deserialize, Serialize};

// TODO: permissions policy for which dispatchers the executor is willing to receive commands
// from.
#[derive(Deserialize)]
pub struct Input {
    // The target whose outstanding commands will be received.
    pub target_name: String,
    // Command batch ids to not return (because the client already knows about them).
    pub exclude_batches: Vec<String>,
    // When polling for commands, clients also specify which constraint groups they
    // are a part of.
    pub group_membership: Vec<String>,
    // Max time that the client is willing to wait for the long poll to return.
    pub timeout_millis: usize
}

#[derive(Serialize)]
pub struct Output {
    // Every returned batch represents an independent work track.
    pub command_batches: Vec<Batch>
}

#[derive(Serialize)]
pub struct Batch {
    // Unique id of the command batch.
    pub id: String,
    // The commands in the batch, in the order in which they should be serially
    // executed. Order is enforced by the dispatch service in the per-command
    // operations.
    pub commands: Vec<Command>,
}

// Note that commands returned by ReceiveCommands are a lot slimmer than the full
// command specified in DispatchCommands. That's because the client is told whether
// to retry, continue, halt etc in CompleteCommand, so it doesn't need to know about
// retry settings, max attempts, etc itself.
#[derive(Serialize)]
pub struct Command {
    // Index. Not guaranteed to start at 0 for a given batch.
    pub index: usize,
    // Freeform command name.
    pub name: String,
    // Freeform command data.
    pub data: String,
    // How often clients should heartbeat the command when it is executing but not
    // completed.
    pub heartbeat_interval_millis: usize
}

pub async fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("receive_commands called"))
}