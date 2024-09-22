use tokio::task::JoinHandle;

use crate::{
    configuration::module_configuration::ModuleConfiguration, modules::module::ModuleTrait,
};

pub struct EchoModule {
    pub(crate) configuration: ModuleConfiguration,
}

impl ModuleTrait for EchoModule {
    fn new(configuration: ModuleConfiguration) -> Self {
        EchoModule { configuration }
    }

    fn run(self: Box<Self>) -> JoinHandle<()> {
        let rval = tokio::spawn(async {
            // Wait for something to come into my socket.
        });

        rval
    }

    fn get_name(&self) -> &'static str {
        "echo"
    }

    fn set_outbox(&mut self, _outbox: Option<async_channel::Sender<crate::messaging::message::Message>>) {
        unimplemented!("Echo module does not need an outbox. It is a sink module");
    }

    fn set_inbox(&mut self, inbox: Option<async_channel::Receiver<crate::messaging::message::Message>>) {
        self.configuration.inbox = inbox;
    }
}
