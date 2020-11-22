use crate::connection::Connection;
use crate::message::{Message, MessageKind};
use crate::ts_queue::ThreadSafeQueue;
use tokio::prelude::*;

pub struct ClientInterface<T> {
    messages_in: ThreadSafeQueue<T>,
    connection: Connection<T>,
}

impl<T: MessageKind> ClientInterface<T> {
    pub async fn connect(
        &mut self,
        host: &str,
        port: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.connection
            .connect_to_server(&format!("{}:{}", host, port))
            .await?;

        Ok(())
    }

    async fn run(&mut self) {
        todo!()
    }

    pub fn is_connected(&self) -> bool {
        self.connection.is_connected()
    }
}
