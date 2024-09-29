use tokio::task::JoinHandle;

use crate::{
    configuration::module_properties::ModuleProperties, modules::module::ModuleTrait,
};

pub struct EchoModule {
    pub(crate) configuration: ModuleProperties,
}

impl ModuleTrait for EchoModule {
    fn new(configuration: ModuleProperties) -> Self {
        EchoModule { configuration }
    }

    fn run(self: Box<Self>) -> JoinHandle<()> {
        tokio::spawn(async {
            if let Some(inbox) = self.configuration.inbox {
                loop {
                    let message = inbox.recv().await;
                    match message {
                        Ok(message) => {
                            println!("Echoing message: {:?}", message.fields.get("data"));
                        }
                        Err(e) => {
                            println!("Error receiving message: {:?}", e);
                            break;
                        }
                    }
                }
            }
        })
    }

    fn set_inbox(
        &mut self,
        inbox: Option<async_channel::Receiver<crate::messaging::message::Message>>,
    ) {
        self.configuration.inbox = inbox;
    }
}
