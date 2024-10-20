use crate::{configuration::module_properties::ModuleProperties, modules::echo_module::EchoModule};

use super::{
    infinite_sender::InfiniteSender, module::ModuleTrait, stdinwriter::StdinWriter,
    tcpsocket::TCPSocketListener, tcpwriter::TCPSocketWriter, udpsocket::UDPSocketListener,
};

pub struct ModulesRegistry;

impl ModulesRegistry {
    pub fn get_module(name: &str, configuration: ModuleProperties) -> Box<dyn ModuleTrait> {
        match name {
            "echo" => Box::new(EchoModule::new(configuration)),
            "udpsocketlistener" => Box::new(UDPSocketListener::new(configuration)),
            "tcpsocketlistener" => Box::new(TCPSocketListener::new(configuration)),
            "infinitesender" => Box::new(InfiniteSender::new(configuration)),
            "tcpwriter" => Box::new(TCPSocketWriter::new(configuration)),
            "stdin" => Box::new(StdinWriter::new(configuration)),
            _ => {
                panic!("Unknown module type: {}", name)
            }
        }
    }
}
