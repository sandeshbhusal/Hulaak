use crate::{
    configuration::module_properties::ModuleProperties,
    modules::echo_module::EchoModule,
};

use super::{module::ModuleTrait, tcpsocket::TCPSocketListener, udpsocket::UDPSocketListener};

pub struct ModulesRegistry;

impl ModulesRegistry {
    pub fn get_module(name: &str, configuration: ModuleProperties) -> Box<dyn ModuleTrait> {
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
