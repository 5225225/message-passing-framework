use common::{Complex, CustomMsg};
use message_passing_framework::message::Message;
use message_passing_framework::server::ServerInterface;

#[tokio::main]
async fn main() {
    let mut server: ServerInterface<CustomMsg> = ServerInterface::new(8080);
    server.start().await;
    let ping = Message::new(CustomMsg::Ping);
    server.ping_loop(ping, 100);
    let mut connection_count = 0;

    loop {
        server.update().await;
        let curr_connection_count = server.connection_count();
        if connection_count != curr_connection_count {
            println!(
                "[Driver] change in connection count: old:{} new:{}",
                connection_count, curr_connection_count
            );
            let ping = Message::new(CustomMsg::Ping);
            server.send_to_all(ping).await;
            connection_count = curr_connection_count;
        }

        if let Some(mut msg) = server.pop_message() {
            println!("popped msg: {:?}", msg.header);
            match msg.header.id {
                CustomMsg::Player(_) => {}
                CustomMsg::Ping => {}
                CustomMsg::Disconnect => {}
                CustomMsg::Interact(_) => {}
                CustomMsg::MovePlayer(_) => {
                    let _: Complex = msg.pull();
                    //println!("parsed bytes for MovePlayer: {:#?}", parse);
                }
            }
        }
    }
}
