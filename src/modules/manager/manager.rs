use std::cell::OnceCell;

use crate::{configuration::global_configuration::GlobalConfiguration, modules::module::ModuleTrait};

pub struct Manager {
    configuration: GlobalConfiguration,
}

impl Manager {
    fn new(configuration: GlobalConfiguration) -> Self {
        Manager {
            configuration,
        }
    }

    fn run(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async {
            // Parse the configuration for the modules, and start them.
        })
    }
}
