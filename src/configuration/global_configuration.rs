use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::module_configuration::ModuleConfiguration;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalConfiguration {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub stage_id: uuid::Uuid,

    // List of modules.
    pub modules: HashMap<String, ModuleConfiguration>,

    // List of routes.
    pub routes: HashMap<String, RouteConfiguration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RouteCardinality {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteConfiguration {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,

    pub from: RouteCardinality,
    pub to: RouteCardinality,
}
