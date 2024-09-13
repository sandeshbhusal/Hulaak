use async_channel::{Receiver, Sender};
use tokio::{join, select, task::JoinHandle};

type Message = String;

#[derive(Debug)]
struct Module {
    name: &'static str,
    input_channel: Option<Receiver<Message>>,
    output_channel: Option<Sender<Message>>,
}

impl Module {
    fn new(name: &'static str) -> Self {
        Module {
            name,
            input_channel: None,
            output_channel: None,
        }
    }

    async fn run(self) -> Result<JoinHandle<()>, ()> {
        if let Some(input_channel) = self.input_channel {
            let task = tokio::spawn(async move {
                'outerloop: loop {
                    select! {
                        msg = input_channel.recv() => {
                            match msg {
                                Ok(received) => {
                                    println!("{} received: {}", self.name, received);
                                    if let Some(output_channel) = &self.output_channel {
                                        output_channel.send(received).await.unwrap();
                                    }                                },
                                Err(recverr) => {
                                    println!("{} received error: {:?}", self.name, recverr);
                                    break 'outerloop;
                                },
                            }
                        }
                    }
                }
            });

            Ok(task)
        } else {
            println!("No input channel set for module.");
            Err(())
        }
    }

    fn set_input_channel(&mut self, input_channel: Receiver<Message>) {
        self.input_channel = Some(input_channel);
    }

    fn set_downstream(&mut self, downstream: &mut Module) {
        let (tx, rx) = async_channel::unbounded();
        self.output_channel = Some(tx);
        downstream.set_input_channel(rx);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = async_channel::unbounded();
    let mut ping_module = Module::new("Ping");
    let mut pong_module = Module::new("Pong");

    ping_module.set_input_channel(rx);
    ping_module.set_downstream(&mut pong_module);

    let ping_module_handle = tokio::spawn(async move {
        ping_module.run().await.unwrap();
    });

    let pong_module_handle = tokio::spawn(async move {
        pong_module.run().await.unwrap();
    });

    let sender_handle = tokio::spawn(async move {
        let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(1));
        let mut x = 0;
        while x < 2 {
            tx.send("Hello".to_string()).await.unwrap();
            ticker.tick().await;
            x += 1;
        }
    });

    // println!("Sent init message");
    let _ = join!(sender_handle, ping_module_handle, pong_module_handle);
}
