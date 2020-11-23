use message_passing_framework::client::ClientInterface;
use message_passing_framework::message::{Message, MessageKind};
use tokio::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum CustomMsg {
    Interact(usize),
    MovePlayer(usize),
}

impl std::fmt::Display for CustomMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomMsg::Interact(id) => write!(f, "Interact({})", id),
            CustomMsg::MovePlayer(id) => write!(f, "MovePlayer({})", id),
        }
    }
}

impl MessageKind for CustomMsg {}

#[derive(Clone, Copy, Debug)]
struct F2 {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    a: u32,
    b: bool,
    c: f32,
    d: [F2; 2],
}

#[tokio::main]
async fn main() {
    let mut client: ClientInterface<CustomMsg> = ClientInterface::new();
    println!("{:?}", client.connect("127.0.0.1", 8080).await);
    loop {}
}
