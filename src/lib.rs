use std::io::{BufRead, StdoutLock, Write};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: MessageBody<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBody<Payload> {
    pub msg_id: Option<i32>,
    pub in_reply_to: Option<i32>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait Node<S, Payload> {
    fn from_init(state: S, init: Init) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn main_loop<S, N, P>(init_state: S) -> anyhow::Result<()>
where
    N: Node<S, P>,
    P: DeserializeOwned, // could be changed to de::DeserializeOwned
{
    let stdin = std::io::stdin().lock();
    let mut input_lines = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<InitPayload> = serde_json::from_str(
        &input_lines
            .next()
            .expect("no init message received")
            .context("failed to read init message from stdin")?,
    )
    .context("failed to parse init message")?;

    let InitPayload::Init(init) = init_msg.body.payload else {
        panic!("first message should be init");
    };

    let mut node: N = Node::from_init(init_state, init).context("failed to create node object")?;

    let reply = Message {
        src: init_msg.dst,
        dst: init_msg.src,
        body: MessageBody {
            msg_id: Some(0),
            in_reply_to: init_msg.body.msg_id,
            payload: InitPayload::InitOk
        },
    };

    serde_json::to_writer(&mut stdout, &reply).context("failed to write init reply to stdout")?;
    stdout.write_all(b"\n").context("failed to write trailing newline")?;

    for line in input_lines {
        let line = line.context("failed to parse input line")?;
        let input: Message<P> = serde_json::from_str(&line)
            .context("failed to parse message from input line")?;
        node.step(input, &mut stdout)
            .context("failed to step node")?;
    }

    Ok(())
}
