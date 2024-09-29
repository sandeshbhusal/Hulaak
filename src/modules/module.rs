use async_channel::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::{configuration::module_properties::ModuleProperties, messaging::message::Message};

pub trait ModuleTrait: Sync + Send {
    fn new(configuration: ModuleProperties) -> Self
    where
        Self: Sized;

    fn set_outbox(&mut self, _outbox: Option<Sender<Message>>) {
        unimplemented!("This module does not support an outbox");
    }
    fn set_inbox(&mut self, _inbox: Option<Receiver<Message>>) {
        unimplemented!("This module does not support an inbox");
    }
    fn run(self: Box<Self>) -> JoinHandle<()>;
}
