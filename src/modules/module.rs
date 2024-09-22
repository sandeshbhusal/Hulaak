use async_channel::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::{
    configuration::module_configuration::ModuleConfiguration, messaging::message::Message,
};

pub trait ModuleTrait: Sync + Send {
    fn get_name(&self) -> &'static str;
    fn new(configuration: ModuleConfiguration) -> Self
    where
        Self: Sized;
    fn set_outbox(&mut self, outbox: Option<Sender<Message>>);
    fn set_inbox(&mut self, inbox: Option<Receiver<Message>>);
    fn run(self) -> JoinHandle<()>;
}
