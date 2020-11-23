use crate::connection::Connection;
use crate::message::{Message, MessageKind};
use crate::Command;
use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

pub struct ServerInterface<T: MessageKind> {
    port: u16,
    messages_in: Arc<RwLock<VecDeque<Message<T>>>>,
    connections: Arc<RwLock<Vec<Connection<T>>>>,
    id_counter: usize,
}

impl<T: MessageKind> ServerInterface<T> {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            messages_in: Arc::new(RwLock::new(VecDeque::new())),
            connections: Arc::new(RwLock::new(vec![])),
            id_counter: 10000,
        }
    }

    pub async fn start(&mut self) {
        self.listen_for_connections();
        self.event_loop().await;
    }

    pub async fn event_loop(&mut self) {
        loop {
            if self.messages_in.read().expect("poisoned lock").len() > 0 {
                let mut write = self.messages_in.write().expect("poisoned lock");
                while let Some(msg) = write.pop_front() {
                    println!("[Server] got msg {}", msg);
                }
            }

            for connection in self.connections.write().unwrap().iter_mut() {
                connection.ping().await;
            }
        }
    }

    pub fn stop(&mut self) {
        todo!()
    }

    fn listen_for_connections(&self) {
        let port = self.port;
        let connections = self.connections.clone();
        let messages_in = self.messages_in.clone();

        tokio::spawn(async move {
            let addr = format!("127.0.0.1:{}", port);
            println!("[Server] starting on {}", addr);
            let listener = match TcpListener::bind(addr).await {
                Ok(listener) => listener,
                Err(_) => unimplemented!(),
            };

            loop {
                let (socket, _) = match listener.accept().await {
                    Ok(accept) => accept,
                    Err(_) => unimplemented!(),
                };

                println!("[Server] new client on {:#?}", socket.peer_addr().unwrap());
                let mut write = connections.write().expect("poisoned lock");
                let mut connection = Connection::from_stream(messages_in.clone(), socket);
                connection.start_read_loop();
                write.push(connection);
            }
        });
    }
}
