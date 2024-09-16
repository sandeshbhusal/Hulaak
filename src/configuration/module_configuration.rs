use std::collections::HashMap;

use uuid::Uuid;

use async_channel::{Sender, Receiver};
use serde::{Deserialize, Serialize};

pub type PeerID = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ModuleType {
    Input,
    Output,
    Processor
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum ModuleAddressType {
    #[default]
    Managed,

    LocalPeerManaged(PeerID),
    RemotePeerManaged(PeerID),
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleDefinition{
    #[serde(default = "Uuid::new_v4")]
    pub uuid: uuid::Uuid,

    pub name: String,
    pub module: String,
    pub module_type: ModuleType,
    pub description: String,
    pub address_type: ModuleAddressType,

    pub module_settings: HashMap<String, serde_json::Value>,

    #[serde(skip)]
    pub inbox: Option<Receiver<String>>,
    #[serde(skip)]
    pub outbox: Option<Sender<String>>,
}
