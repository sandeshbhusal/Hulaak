use std::collections::HashMap;

use tokio::net::UdpSocket;

use crate::{
    configuration::module_configuration::ModuleConfiguration, messaging::message::Message,
    modules::module::ModuleTrait,
};

use super::UdpSocketConfiguration;

pub struct UDPSocketListener {
    properties: ModuleConfiguration,
    configuration: UdpSocketConfiguration,
}

impl ModuleTrait for UDPSocketListener {
    fn new(configuration: ModuleConfiguration) -> Self {
        // Convert module config to udp socket config.
        let _config = configuration.module_settings.clone();
        let serialized_config = serde_json::to_string(&_config).unwrap();

        // Convert the serialized config to module config.
        let module_config: UdpSocketConfiguration = serde_json::from_str(&serialized_config)
            .expect("Error configuring the filechange module");

        UDPSocketListener {
            properties: configuration,
            configuration: module_config,
        }
    }

    fn set_outbox(
        &mut self,
        outbox: Option<async_channel::Sender<crate::messaging::message::Message>>,
    ) {
        self.properties.outbox = outbox;
    }

    fn set_inbox(
        &mut self,
        _inbox: Option<async_channel::Receiver<crate::messaging::message::Message>>,
    ) {
        unimplemented!("UDPSocketListener should not have an input.");
    }

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let socket = UdpSocket::bind(format!(
                "{}:{}",
                self.configuration.address, self.configuration.port
            ))
            .await
            .expect("Failed to create UDP socket");

            let mut buffer = Vec::with_capacity(self.configuration.buffer_size);
            loop {
                // Connection established
                while let Ok((size, _src)) = socket.recv_from(&mut buffer).await {
                    let mut event = HashMap::new();
                    // Run parsers on the buffer to split it into key-value pairs.
                    event.insert("message_size".into(), (size as i64).into());
                    event.insert("message".into(), "Hola!".into());

                    let message = Message::new(event);
                    if let Some(outbox) = &self.properties.outbox {
                        outbox.send(message).await.expect("Failed to send message");
                    }
                }
                // Connection closed.
            }
        })
    }
}
