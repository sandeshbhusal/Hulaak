use tokio::task::JoinHandle;

use crate::configuration::module_configuration::ModuleDefinition;

pub trait ModuleTrait: Sync + Send {
    fn new(configuration: ModuleDefinition) -> Self where Self: Sized;
    fn run(self) -> JoinHandle<()>;
}