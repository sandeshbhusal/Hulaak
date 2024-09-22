use async_channel::Receiver;
use serde::{Deserialize, Serialize};

pub mod echo_module;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Configuration {
    #[serde(skip)]
    input_channel: Option<Receiver<String>>
}
