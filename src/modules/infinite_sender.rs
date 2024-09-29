use std::collections::HashMap;

use crate::{configuration::module_properties::ModuleProperties, messaging::message::Message};

use super::module::ModuleTrait;

pub struct InfiniteSender {
    pub(crate) properties: ModuleProperties,
}

impl ModuleTrait for InfiniteSender {
    fn new(configuration: ModuleProperties) -> Self
    where
        Self: Sized,
    {
        Self {
            properties: configuration,
        }
    }

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut counter = 1_000_000;

            while counter > 0 {
                let message = Message {
                    fields: HashMap::new(),
                };

                if let Some(outbox) = &self.properties.outbox {
                    outbox.send(message).await.unwrap();
                }
                counter -= 1;
            }

            println!("InfiniteSender: Done sending messages");
        })
    }

    fn set_outbox(
        &mut self,
        _outbox: Option<async_channel::Sender<crate::messaging::message::Message>>,
    ) {
        self.properties.outbox = _outbox;
    }
}
