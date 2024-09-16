use tokio::task::JoinHandle;

use crate::{configuration::module_configuration::ModuleDefinition, modules::module::ModuleTrait};

pub struct PongModule {
    pub(crate) configuration: ModuleDefinition,
}

impl ModuleTrait for PongModule {
    fn new(configuration: ModuleDefinition) -> Self {
        PongModule { configuration }
    }

    fn run(self) -> JoinHandle<()> {
        todo!()
    }
}
