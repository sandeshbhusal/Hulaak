use std::{collections::HashMap, default};

use uuid::Uuid;

use async_channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

pub type PeerID = String;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum ModuleAddressType {
    #[default]
    Managed,

    LocalPeerManaged(PeerID),
    RemotePeerManaged(PeerID),
}

#[allow(unused)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct ModuleConfiguration {
    #[serde(default = "Uuid::new_v4")]
    pub uuid: uuid::Uuid,

    pub module: String,
    pub description: Option<String>,
    pub address_type: Option<ModuleAddressType>,

    pub module_settings: Option<HashMap<String, serde_json::Value>>,

    #[serde(skip)]
    pub inbox: Option<Receiver<String>>,
    #[serde(skip)]
    pub outbox: Option<Sender<String>>,
}
