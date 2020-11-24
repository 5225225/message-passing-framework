use message_passing_framework::message::{Message, MessageKind};
use message_passing_framework::server::ServerInterface;

#[derive(Clone, Copy, Debug)]
pub enum CustomMsg {
    Ping,
    Interact(usize),
    MovePlayer(usize),
}

impl std::fmt::Display for CustomMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomMsg::Ping => write!(f, "Ping"),
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
    let mut server: ServerInterface<CustomMsg> = ServerInterface::new(8080);
    server.start().await;
    let mut connection_count = 0;

    loop {
        server.update().await;
        let ping = Message::new(CustomMsg::Ping);
        if connection_count != server.connection_count() {
            server.send_to_all(ping).await;
            connection_count = server.connection_count();
        }

        if let Some(mut msg) = server.pop_message() {
            match msg.header.id {
                CustomMsg::Ping => {}
                CustomMsg::Interact(_) => {}
                CustomMsg::MovePlayer(_) => {
                    let parse: Complex = msg.pull();
                    println!("parsed bytes for MovePlayer: {:#?}", parse);
                }
            }
        }
    }
}
