use std::{
    future::Future,
    io::{Error, Read, Write},
    net::UdpSocket,
    os::unix::net::UnixStream,
};

use anyhow::Result;
use async_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use tokio::{net::UnixSocket, select, task::JoinHandle};

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

enum Senders {
    Udp(UdpSocket),
    Uds(UnixSocket),
    AsyncChannel(Sender<Message>),
}

enum Receivers {
    Udp(UdpSocket),
    Uds(UnixStream),
    AsyncChannel(Receiver<Message>),
}

struct MailBox {
    in_box: Receivers,
    out_box: Option<Senders>,
}

impl MailBox {
    async fn send_message(&mut self, message: Message) {
        if let Some(out_box) = &mut self.out_box {
            match out_box {
                Senders::Udp(socket) => {
                    socket
                        .send(bincode::serialize(&message).unwrap().as_ref())
                        .unwrap();
                }

                Senders::AsyncChannel(sender) => {
                    sender.send(message).await.unwrap();
                }

                Senders::Uds(_) => todo!(),
            }
        } else {
            println!("No outbox found for module");
        }
    }

    async fn receive_message(&mut self) -> Result<Option<Message>> {
        let message = match &mut self.in_box {
            Receivers::Udp(socket) => {
                let mut buf = [0; 1024];
                let (size, _) = socket.recv_from(&mut buf)?;
                if size == 0 {
                    None
                } else {
                    let message = std::str::from_utf8(&buf[..size])?;
                    Some(Message::Normal(message.to_string()))
                }
            }

            Receivers::AsyncChannel(receiver) => Some(receiver.recv().await.expect("Async failed")),
            Receivers::Uds(_) => todo!(),
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
                    .send_message(Message::Normal("Ping".to_string()))
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

struct Pong;
impl Actor for Pong {
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
    let (tx2, rx2) = async_channel::unbounded();

    let mailbox = MailBox {
        in_box: Receivers::AsyncChannel(rx),
        out_box: Some(Senders::AsyncChannel(tx2)),
    };

    let pong_mailbox = MailBox {
        in_box: Receivers::AsyncChannel(rx2),
        out_box: Some(Senders::AsyncChannel(tx.clone())),
    };

    let ping = Ping;
    let pong = Pong;
    let jh = ping.run(mailbox);
    let kh = pong.run(pong_mailbox);

    // Ping should receive one message :)
    tx.send(Message::Normal("Hello!".to_string()))
        .await
        .unwrap();

    println!("Starting..");
    let _ = tokio::join!(jh, kh);
    loop {}
}
