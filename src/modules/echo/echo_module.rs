use tokio::task::JoinHandle;

use crate::{configuration::module_configuration::ModuleConfiguration, modules::module::ModuleTrait};

pub struct PingModule {
    pub(crate) configuration: ModuleConfiguration
}

impl ModuleTrait for PingModule {
    fn new(configuration: ModuleConfiguration) -> Self {
        PingModule {
            configuration
        }
    }

    fn run(self) -> JoinHandle<()> {
        let rval = tokio::spawn(async {
            // Wait for something to come into my socket.
        });

        rval
    }

    fn get_name() -> &'static str {
        "ping"
    }
}
