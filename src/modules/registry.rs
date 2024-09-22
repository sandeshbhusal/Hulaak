use crate::{
    configuration::module_configuration::ModuleConfiguration,
    modules::echo::echo_module::EchoModule,
};

use super::{filechange::lib::FileChangeWatcherModule, module::ModuleTrait};

pub struct ModulesRegistry;

impl ModulesRegistry {
    pub fn get_module(name: &str, configuration: ModuleConfiguration) -> Box<dyn ModuleTrait> {
        match name {
            "echo" => Box::new(EchoModule::new(configuration)),
            "filechangewatcher" => Box::new(FileChangeWatcherModule::new(configuration)),
            _ => {
                panic!("Unknown module type: {}", name)
            }
        }
    }
}
