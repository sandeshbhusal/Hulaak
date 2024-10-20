use std::{collections::HashMap, time::Duration};

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
            // Read data from stdin, writing it to the outbox as newlines are
            // encountered.

            let mut buffer = String::new();
            let stdin = std::io::stdin(); // We get `Stdin` here.
            loop {
                stdin
                    .read_line(&mut buffer)
                    .expect("Error reading from stdin");

                if let Some(outbox) = self.properties.outbox.clone() {
                    let timestamp = format!("{:?}", std::time::SystemTime::now());
                    let mut map = HashMap::new();
                    map.insert("data".into(), buffer.clone().into());
                    map.insert("timestamp".into(), timestamp.into());

                    outbox
                        .send(Message::new(map))
                        .await
                        .expect("Error sending message internally");
                }

                // Sleep for a while here, suspending this thread.
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        })
    }
}
