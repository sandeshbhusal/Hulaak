use std::collections::HashMap;

use tokio::io::{self, AsyncBufReadExt};

use crate::{
    configuration::module_properties::ModuleProperties, messaging::message::Message,
    modules::module::ModuleTrait,
};

pub struct StdinWriter {
    pub(crate) properties: ModuleProperties,
}

impl ModuleTrait for StdinWriter {
    fn new(configuration: ModuleProperties) -> Self
    where
        Self: Sized,
    {
        Self {
            properties: configuration,
        }
    }

    fn set_outbox(
        &mut self,
        outbox: Option<async_channel::Sender<crate::messaging::message::Message>>,
    ) {
        self.properties.outbox = outbox;
    }

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
            let mut buffer = String::new();

            loop {
                buffer.clear(); // Clear the buffer before each read
                match stdin.read_line(&mut buffer).await {
                    Ok(0) => break, // End of input
                    Ok(_) => {
                        if let Some(outbox) = self.properties.outbox.clone() {
                            let timestamp = format!("{:?}", std::time::SystemTime::now());
                            let mut map = HashMap::new();
                            map.insert("data".into(), buffer.trim().to_string().into());
                            map.insert("timestamp".into(), timestamp.into());

                            if let Err(e) = outbox.send(Message::new(map)).await {
                                eprintln!("Error sending message internally: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }
        })
    }
}
