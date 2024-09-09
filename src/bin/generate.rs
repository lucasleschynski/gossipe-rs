use gossipe_rs::*;

use std::io::{StdoutLock, Write};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk { id: String },
}

struct GenerateNode {
    node: String,
    id: i32,
}

impl Node<(), Payload> for GenerateNode {
    fn from_init(_state: (), init: Init) -> anyhow::Result<Self>
        where
            Self: Sized {
        Ok(GenerateNode {
            node: init.node_id,
            id: 1
        })
    }

    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Generate => {
                let response = Message { 
                    src: input.dst,
                    dst: input.src,
                    body: MessageBody {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::GenerateOk { id: format!("{}-{}", self.node, self.id) }
                    }
                };
                serde_json::to_writer(&mut *output, &response)
                    .context("failed to write response to stdout")?;
                output.write_all(b"\n")
                    .context("failed to write newline")?;
                self.id += 1;
            }
            Payload::GenerateOk { .. } => {}
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<_, GenerateNode, _>(())
}