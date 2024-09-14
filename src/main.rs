use std::{
    io::{Error, Read, Write},
    net::UdpSocket,
};

use anyhow::Result;
use async_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use tokio::{select, task::JoinHandle};

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Normal(String),
    HeartBeat,
    ShutDown,
}

enum MessageReply {
    Ok,
    Fail(Error),
    NotImplemented,
}

enum Transport {
    Udp(UdpSocket),
    File(std::fs::File),
    AsyncChannel {
        sender: Sender<Message>,
        receiver: Receiver<Message>,
    },
}

struct MailBox {
    in_box: Transport,
    out_box: Transport,
}

impl MailBox {
    fn new_mailbox(in_box: Transport, out_box: Transport) -> Self {
        MailBox { in_box, out_box }
    }

    async fn send_message(&mut self, message: Message) {
        match &mut self.out_box {
            Transport::Udp(socket) => {
                socket
                    .send(bincode::serialize(&message).unwrap().as_ref())
                    .unwrap();
            }

            Transport::File(file) => {
                file.write_all(bincode::serialize(&message).unwrap().as_ref())
                    .unwrap();
            }

            Transport::AsyncChannel { sender, .. } => {
                sender.send(message).await.unwrap();
            }
        }
    }

    async fn receive_message(&mut self) -> Result<Option<Message>> {
        let message = match &mut self.in_box {
            Transport::Udp(socket) => {
                let mut buf = [0; 1024];
                let (size, _) = socket.recv_from(&mut buf)?;
                if size == 0 {
                    None
                } else {
                    let message = std::str::from_utf8(&buf[..size])?;
                    Some(Message::Normal(message.to_string()))
                }
            }

            // Do stuff like inotify.
            Transport::File(ref mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                Some(Message::Normal(buf))
            }

            Transport::AsyncChannel { receiver, .. } => {
                Some(receiver.recv().await.expect("Async failed"))
            }
        };

        Ok(message)
    }
}

trait Actor {
    async fn run(self, mailbox: MailBox) -> JoinHandle<()>;
    async fn handle_message(&self, mailbox: &mut MailBox) -> MessageReply;
}

struct Ping;
struct Pong;

impl Actor for Ping {
    async fn run(self, mut mailbox: MailBox) -> JoinHandle<()> {
        let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(1));
        let joinhandle = tokio::spawn(async move {
            loop {
                select! {
                    _ = mailbox.receive_message() => {
                        self.handle_message(&mut mailbox).await;
                    },

                    _ = ticker.tick() => {
                        mailbox.send_message(Message::HeartBeat).await;
                    }
                }
            }
        });

        joinhandle
    }

    async fn handle_message(&self, mailbox: &mut MailBox) -> MessageReply {
        match mailbox.receive_message().await {
            Ok(Some(message)) => match message {
                Message::Normal(message) => {
                    println!("Received message: {}", message);
                }
                Message::HeartBeat => {}
                Message::ShutDown => {}
            },
            Ok(None) => {
                println!("No message received.")
            }
            Err(error) => println!("Error: {}", error),
        }

        MessageReply::Ok
    }
}

#[tokio::main]
async fn main() {
    let ping = Ping;
}
