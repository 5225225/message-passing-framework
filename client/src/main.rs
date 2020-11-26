use common::{Complex, CustomMsg, F2};
use message_passing_framework::client::ClientInterface;
use message_passing_framework::message::Message;

#[tokio::main]
async fn main() {
    let mut client: ClientInterface<CustomMsg> = ClientInterface::new();
    println!(
        "Connectinon status: {:?}",
        client.connect("127.0.0.1", 8080).await
    );
    //println!("size of CustomMsg {:?}", std::mem::size_of::<CustomMsg>());
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
