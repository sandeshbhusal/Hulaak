use crate::{
    configuration::module_configuration::ModuleConfiguration,
    modules::echo::echo_module::EchoModule,
};

use super::{
    module::ModuleTrait, tcpsocket::tcpsocket::TCPSocketListener,
    udpsocket::udpsocket::UDPSocketListener,
};

pub struct ModulesRegistry;

impl ModulesRegistry {
    pub fn get_module(name: &str, configuration: ModuleConfiguration) -> Box<dyn ModuleTrait> {
        match name {
            "echo" => Box::new(EchoModule::new(configuration)),
            "udpsocketlistener" => Box::new(UDPSocketListener::new(configuration)),
            "tcpsocketlistener" => Box::new(TCPSocketListener::new(configuration)),
            _ => {
                panic!("Unknown module type: {}", name)
            }
        }
    }
}
