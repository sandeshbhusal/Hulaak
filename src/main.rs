use tokio::{
    select,
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
    time,
};

#[derive(Debug)]
enum Message {}

#[derive(Debug)]
struct Module {
    name: &'static str,
    output: Sender<Message>,
    input: Receiver<Message>,
}

impl Module {
    async fn run(mut self) -> JoinHandle<()> {
        let mut ticker = time::interval(time::Duration::from_secs(1));
        let handle = tokio::spawn(async move {
            loop {
                select! {
                    Some(msg) = self.input.recv() => {
                        println!("{}: Received some message: {:?}", self.name, msg);
                        // Write back the same message.
                        let _ = self.output.send(msg).await;
                    }
                    _ = ticker.tick() => {
                        // Do something
                        println!("{}: Handling tick.", self.name)
                    }
                }
            }
        });

        handle
    }

    fn new(name: &'static str) -> Self {
        let (output, input) = tokio::sync::mpsc::channel(100);
        Self {
            output,
            input,
            name,
        }
    }
}

fn main() {
    let module1 = Module::new("Ping");
    let module2 = Module::new("Pong");

    let handle = Module::run(module1);
    let handle2 = Module::run(module2);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        tokio::select! {
            _ = handle.await => {
                println!("Module finished.");
            }
            _ = handle2.await => {
                println!("Module2 finished.");
            }
        }
    });

    loop {}
}
