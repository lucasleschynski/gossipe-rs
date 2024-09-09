use gossipe_rs::*;

use std::collections::{HashMap, HashSet};
use std::io::{StdoutLock, Write};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    BroadcastPayload,
    ReadPayload,
    TopologyPayload,
}

enum BroadcastPayload {
    Broadcast { message: i32 },
    BroadcastOk,
}

enum ReadPayload {
    Read,
    ReadOk { messages: HashSet<i32> },
}

enum TopologyPayload {
    Topology { topology: HashMap<String, Vec<String>> },
    TopologyOk,
}


struct BroadcastNode {
    id: i32,
}

fn main() -> anyhow::Result<()> {
    Ok(())
}