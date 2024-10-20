use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{configuration::module_properties::ModuleProperties, modules::module::ModuleTrait};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TCPWriterConfiguration {
    address: String,
    port: u16,
}

pub struct TCPSocketWriter {
    pub(crate) properties: ModuleProperties,
    configuration: TCPWriterConfiguration,
}

impl ModuleTrait for TCPSocketWriter {
    fn new(configuration: ModuleProperties) -> Self
    where
        Self: Sized,
    {
        // Convert module config to tcp socket config.
        let config = configuration.module_settings.clone();
        let serialized_config = serde_json::to_string(&config).unwrap();

        // Convert the serialized config to module config.
        let module_config: TCPWriterConfiguration = serde_json::from_str(&serialized_config)
            .expect("Error configuring the tcpsocket module");

        Self {
            properties: configuration,
            configuration: module_config,
        }
    }

    fn set_inbox(&mut self, inbox: Option<async_channel::Receiver<crate::messaging::message::Message>>) {
        self.properties.inbox = inbox;
    }

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        // Check that self.inbox exists.
        if self.properties.inbox.is_none() {
            panic!("TCPWriter requires an inbox to function.");
        }

        // SAFETY: safe to unwrap - checks above.
        let inbox = self.properties.inbox.unwrap();
        let address = format!("{}:{}", self.configuration.address, self.configuration.port);

        tokio::spawn(async move {
            // Generate a TCP Writer.
            let mut remote_socket = TcpStream::connect(address)
                .await
                .expect("Error binding to remote socket");

            while let Ok(message) = inbox.try_recv() {
                let data_to_send: Vec<u8> = serde_json::to_vec(
                    message.fields.get("data").unwrap_or(&json!("default text")),
                )
                .unwrap_or_else(|_| {
                    println!("Error deserializing bytes from message: {:?}", message);
                    vec![]
                });

                // Write data!
                remote_socket
                    .write_all(&data_to_send)
                    .await
                    .expect("Could not write message to remote stream!");
            }
        })
    }
}
