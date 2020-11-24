use message_passing_framework::client::ClientInterface;
use message_passing_framework::message::{Message, MessageKind};

#[derive(Clone, Copy, Debug)]
pub enum CustomMsg {
    Ping,
    Interact(usize),
    MovePlayer(usize),
    Player(usize),
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
    println!("size of CustomMsg {:?}", std::mem::size_of::<CustomMsg>());
    let mut message: Message<CustomMsg> = Message::new(CustomMsg::MovePlayer(3413));

    message.push(Complex {
        a: 2,
        b: true,
        c: 3.14,
        d: [
            F2 { x: 1., y: 2. },
            F2 {
                x: -213.234,
                y: 514.4223,
            },
        ],
    });

    client.send(message).await.unwrap();

    loop {}
}
