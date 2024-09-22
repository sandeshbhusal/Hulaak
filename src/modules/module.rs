use tokio::task::JoinHandle;

use crate::configuration::module_configuration::ModuleConfiguration;

pub trait ModuleTrait: Sync + Send {
    fn get_name(&self) -> &'static str;
    fn new(configuration: ModuleConfiguration) -> Self where Self: Sized;
    fn run(self) -> JoinHandle<()>;
}
