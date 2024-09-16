use std::collections::HashMap;

use anyhow::Result;
use configuration::global_configuration::GlobalConfiguration;
use modules::{module::ModuleTrait, ping_module::PingModule, pong_module::PongModule};

mod configuration;
mod modules;

const CONFIGURATION: &str = r#"
# Global Configuration
version = "1.3.0"
local_port = 8080
allow_sync = true

# Module Configurations
[[modules]]
name = "ping_1"
module = "ping"
module_type = "Input"
description = "Ping Module"
address_type = "Managed"

[[modules]]
name = "pong_1"
module = "pong"
module_type = "Processor"
description = "Pong Module"
address_type = "Managed"

# Route Configurations
[[routes]]
name = "ping_to_pong"
from = { Single = "ping_1" }
to = { Single = "pong_1" }
"#;

#[tokio::main]
async fn main() -> Result<()> {
    let parsedconfig: GlobalConfiguration = toml::from_str(CONFIGURATION)?;

    let mut configured_modules = HashMap::new();

    for module_definition in parsedconfig.modules.iter() {
        let module_to_use: Box<dyn ModuleTrait> = match module_definition.module.as_str() {
            "ping" => Box::new(PingModule::new(module_definition.clone())),
            "pong" => Box::new(PongModule::new(module_definition.clone())),
            _ => panic!("Unknown module type"),
        };

        // We have a configured module now.
        configured_modules.insert(module_definition.name.clone(), module_to_use);
    }

    Ok(())
}
