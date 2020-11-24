use crate::message::{Message, MessageKind};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;

pub struct Connection<T: MessageKind> {
    messages_in: Arc<Mutex<VecDeque<Message<T>>>>,
    messages_out: Arc<Mutex<VecDeque<Message<T>>>>,

    is_connected: bool,

    read_stream: Option<ReadHalf<tokio::net::TcpStream>>,
    write_stream: Option<WriteHalf<tokio::net::TcpStream>>,
}

impl<T: MessageKind> Connection<T> {
    pub fn new(messages_in: Arc<Mutex<VecDeque<Message<T>>>>) -> Self {
        let messages_out = Arc::new(Mutex::new(VecDeque::new()));

        Self {
            messages_in,
            messages_out,
            is_connected: false,
            write_stream: None,
            read_stream: None,
        }
    }

    pub fn from_stream(
        messages_in: Arc<Mutex<VecDeque<Message<T>>>>,
        stream: tokio::net::TcpStream,
    ) -> Self {
        let messages_out = Arc::new(Mutex::new(VecDeque::new()));

        let (read_stream, write_stream) = tokio::io::split(stream);

        Self {
            messages_in,
            messages_out,
            is_connected: false,
            write_stream: Some(write_stream),
            read_stream: Some(read_stream),
        }
    }

    pub async fn connect_to_server(
        &mut self,
        addr: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send>> {
        match TcpStream::connect(addr).await {
            Ok(stream) => {
                let (read_stream, write_stream) = tokio::io::split(stream);
                self.read_stream = Some(read_stream);
                self.write_stream = Some(write_stream);
                self.is_connected = true;
            }
            Err(e) => return Err(Box::new(e)),
        }
        Ok(())
    }

    pub fn start_read_loop(&mut self) {
        if let Some(mut stream) = self.read_stream.take() {
            let messages_in = self.messages_in.clone();
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    let byte_count = match stream.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("[Read Loop] failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    println!("bytes read: {}", byte_count);
                    let msg: Message<T> = Message::from(&buf[0..byte_count]);
                    println!("Got msg: {:#?}", msg);
                    messages_in.lock().expect("poisoned lock").push_back(msg);
                }
            });
        }
    }

    pub fn start_write_loop(&mut self) {
        if let Some(mut stream) = self.write_stream.take() {
            let messages_out = Arc::clone(&self.messages_out);
            tokio::spawn(async move {
                loop {
                    /* @TODO locking works but underlying data is not shared
                        {
                            loop {
                                match messages_out.try_write() {
                                    Ok(r) => {
                                        println!("[Write Loop] read acquired: {}", r.len());
                                        break;
                                    }
                                    Err(e) => {
                                        eprintln!("[Write Loop] couldn't read queue: {}", e);
                                    }
                                };
                            }
                        }
                    */

                    if messages_out.lock().unwrap().len() > 0 {
                        let next = {
                            let mut write = messages_out.lock().expect("poisoned");
                            write.pop_front()
                        };
                        println!("trying to send: {:?}", next);
                        if let Some(msg) = next {
                            let bytes: Vec<u8> = Vec::from(msg);
                            println!("bytes: {:?}", bytes);

                            if let Err(e) = stream.write(&bytes).await {
                                eprintln!("failed to write to socket; err = {:?}", e);
                                return;
                            }
                        }
                    }
                }
            });
        }
    }

    pub async fn disconnect(&mut self) {
        todo!()
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub async fn send(&mut self, msg: Message<T>) {
        self.messages_out.lock().expect("poisoned").push_back(msg);
    }
}
