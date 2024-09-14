use std::{
    future::Future,
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
    out_box: Option<Transport>,
}

impl MailBox {
    async fn send_message(&mut self, message: Message) {
        if let Some(out_box) = &mut self.out_box {
            match out_box {
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
        } else {
            println!("No outbox found for module");
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

trait Actor: Send + 'static {
    async fn run(self, mut mailbox: MailBox) -> JoinHandle<()>
    where
        Self: Sized,
    {
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                select! {
                    message = mailbox.receive_message() => {
                        if let Ok(Some(message)) = message  {
                            let _ = self.handle_message(message, &mut mailbox).await;
                        } else {
                            // We either got an empty message, or an error.
                            // Send an error to outbox.
                            mailbox.send_message(Message::ShutDown).await;
                        }
                    },

                    _ = ticker.tick() => {
                        mailbox.send_message(Message::HeartBeat).await;
                    }
                }
            }
        })
    }

    fn handle_message(
        &self,
        message: Message,
        mailbox: &mut MailBox,
    ) -> impl Future<Output = MessageReply> + Send;
}

struct Ping;
impl Actor for Ping {
    async fn handle_message(&self, message: Message, mailbox: &mut MailBox) -> MessageReply {
        match message {
            Message::Normal(message) => {
                println!("Received message: {}", message);
                std::io::stdout().flush().unwrap();

                // Send a reply back.
                mailbox
                    .send_message(Message::Normal("Pong".to_string()))
                    .await;
            }
            Message::HeartBeat => {
                return MessageReply::NotImplemented;
            }
            Message::ShutDown => {
                return MessageReply::NotImplemented;
            }
        }

        MessageReply::Ok
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = async_channel::unbounded();

    let mailbox = MailBox {
        in_box: Transport::AsyncChannel {
            sender: tx.clone(),
            receiver: rx.clone(),
        },
        out_box: None,
    };

    let ping = Ping;
    let jh = ping.run(mailbox);

    // Spawn another thread that sends data to ping every second.
    let sender = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(1));
        for _ in 0..5 {
            tx.send(Message::Normal("Hello".to_string())).await.unwrap();
            ticker.tick().await;
        }
    });

    println!("Starting..");
    let _ = tokio::join!(jh, sender);
}
