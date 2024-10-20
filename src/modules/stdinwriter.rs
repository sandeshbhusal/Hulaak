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
            let stdin = io::BufReader::new(tokio::io::stdin()); // Use Tokio's async stdin
            let mut lines = stdin.lines(); // Create an async line stream

            while let Ok(Some(buffer)) = lines.next_line().await {
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
            }
        })
    }
}
