use async_channel::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::{
    configuration::module_properties::ModuleProperties, messaging::message::Message,
};

pub trait ModuleTrait: Sync + Send {
    fn new(configuration: ModuleProperties) -> Self
    where
        Self: Sized;
    fn set_outbox(&mut self, outbox: Option<Sender<Message>>);
    fn set_inbox(&mut self, inbox: Option<Receiver<Message>>);
    fn run(self: Box<Self>) -> JoinHandle<()>;
}
