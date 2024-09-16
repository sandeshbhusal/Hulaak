use tokio::task::JoinHandle;

use crate::{configuration::module_configuration::ModuleDefinition, modules::module::ModuleTrait};

pub struct PingModule {
    pub(crate) configuration: ModuleDefinition
}

impl ModuleTrait for PingModule {
    fn new(configuration: ModuleDefinition) -> Self {
        PingModule {
            configuration
        }
    }
    
    fn run(self) -> JoinHandle<()> {
        todo!()
    }
}
