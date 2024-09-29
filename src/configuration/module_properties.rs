use std::collections::HashMap;

use uuid::Uuid;

use async_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::messaging::message::Message;

#[allow(unused)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct ModuleProperties {
    #[serde(default = "Uuid::new_v4")]
    pub uuid: uuid::Uuid,

    pub module_type: String,
    pub description: Option<String>,

    #[serde(flatten)]
    pub module_settings: Option<HashMap<String, serde_json::Value>>,

    #[serde(skip)]
    pub inbox: Option<Receiver<Message>>,
    #[serde(skip)]
    pub outbox: Option<Sender<Message>>,
}
