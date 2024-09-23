use std::collections::HashMap;

use serde_json::Value;
use tokio::{io::AsyncReadExt, net::TcpListener};

use crate::{
    configuration::module_configuration::ModuleConfiguration, messaging::message::Message,
    modules::module::ModuleTrait,
};

use super::TCPListenerConfiguration;

pub struct TCPSocketListener {
    pub(crate) properties: ModuleConfiguration,
    configuration: TCPListenerConfiguration,
}

impl ModuleTrait for TCPSocketListener {
    fn new(configuration: ModuleConfiguration) -> Self
    where
        Self: Sized,
    {
        // Convert module config to tcp socket config.
        let config = configuration.module_settings.clone();
        let serialized_config = serde_json::to_string(&config).unwrap();

        // Convert the serialized config to module config.
        let module_config: TCPListenerConfiguration = serde_json::from_str(&serialized_config)
            .expect("Error configuring the tcpsocket module");
        Self {
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
        unimplemented!("TCP Socket listener cannot have an inbox. Don't use it as a output module.")
    }

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let listener = TcpListener::bind(format!(
                "{}:{}",
                self.configuration.address, self.configuration.port
            ))
            .await
            .expect("Failed to bind to address");

            loop {
                let (mut socket, _addr) = listener
                    .accept()
                    .await
                    .expect("Failed to accept connection");

                let mut buffer = Vec::with_capacity(self.configuration.buffer_size);

                while let Ok(size) = socket.read(&mut buffer).await {
                    let timestamp = format!("{:?}", std::time::SystemTime::now());
                    let mut event: HashMap<Value, serde_json::Value> = HashMap::new();
                    let content = std::str::from_utf8(&mut buffer[..size]);

                    if let Ok(content) = content {
                        event.insert("endpoint".into(), _addr.to_string().into());
                        event.insert("data".into(), content.to_string().into());
                        event.insert("timestamp".into(), timestamp.into());
                    }

                    if !event.is_empty() {
                        let message = Message::new(event);
                        if let Some(outbox) = &self.properties.outbox {
                            outbox.send(message).await.expect("Failed to send message");
                        }
                    }
                }
            }
        })
    }
}
