use crate::message::{Message, MessageKind};
use crate::ts_queue::ThreadSafeQueue;
use std::cell::RefCell;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub struct Connection<T> {
    messages_in: RefCell<ThreadSafeQueue<T>>,
    messages_out: ThreadSafeQueue<T>,

    stream: Option<TcpStream>,
}

impl<T: MessageKind> Connection<T> {
    pub fn new(messages_in: RefCell<ThreadSafeQueue<T>>) -> Self {
        let messages_out = ThreadSafeQueue::new();

        Self {
            messages_in,
            messages_out,
            stream: None,
        }
    }

    pub async fn connect_to_server(
        &mut self,
        addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(addr).await?;

        self.stream = Some(stream);
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        todo!()
    }

    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    pub async fn send(msg: *const Message<T>) {
        todo!()
    }
}
