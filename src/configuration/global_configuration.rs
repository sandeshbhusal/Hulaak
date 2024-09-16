use serde::{Deserialize, Serialize};

use super::module_configuration::ModuleDefinition;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GlobalConfiguration {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub stage_id: uuid::Uuid,

    pub version: String,
    pub local_port: u16,
    pub allow_sync: bool,

    // List of modules.
    pub modules: Vec<ModuleDefinition>,

    // List of routes.
    pub routes: Vec<RouteConfiguration>
}

#[derive(Debug, Serialize, Deserialize)]
enum RouteCardinality {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct RouteConfiguration {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,

    pub name: String,
    pub from: RouteCardinality,
    pub to: RouteCardinality
}
