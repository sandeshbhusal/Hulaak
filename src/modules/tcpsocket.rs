use std::{collections::HashMap, net::SocketAddr};

use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{
    configuration::module_properties::ModuleProperties, messaging::message::Message,
    modules::module::ModuleTrait,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TCPListenerConfiguration {
    address: String,
    port: u16,
    buffer_size: usize,
}

pub struct TCPSocketListener {
    pub(crate) properties: ModuleProperties,
    configuration: TCPListenerConfiguration,
}

impl ModuleTrait for TCPSocketListener {
    fn new(configuration: ModuleProperties) -> Self
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

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let listener = TcpListener::bind(format!(
                "{}:{}",
                self.configuration.address, self.configuration.port
            ))
            .await
            .expect("Failed to bind to address");

            loop {
                let (socket, _addr) = listener
                    .accept()
                    .await
                    .expect("Failed to accept connection");

                // Need to call handle_connection here to spawn a new task for each connection.
                let cloned_chan = self.properties.outbox.clone();
                tokio::spawn(async move {
                    TCPSocketListener::handle_connection(socket, _addr, cloned_chan).await;
                });
            }
        })
    }
}

impl TCPSocketListener {
    #[allow(dead_code)]
    pub(crate) fn get_settings() -> serde_json::Value {
        let default_settings = TCPListenerConfiguration::default();
        serde_json::to_value(default_settings).expect("Failed to serialize default settings")
    }

    async fn handle_connection(
        mut socket: TcpStream,
        _addr: SocketAddr,
        outbox: Option<async_channel::Sender<Message>>,
    ) {
        println!("Starting to handle connection");
        let mut buffer = Vec::with_capacity(1024);

        while let Ok(size) = socket.read_buf(&mut buffer).await {
            if size == 0 {
                println!("Connection closed");
                break;
            }

            let timestamp = format!("{:?}", std::time::SystemTime::now());
            let mut messages = vec![];

            let content = std::str::from_utf8(&mut buffer[..size]);
            if let Ok(content) = content {
                // Split content into lines.
                for line in content.lines() {
                    let mut event: HashMap<String, serde_json::Value> = HashMap::new();
                    event.insert("data".into(), line.into());
                    event.insert("timestamp".into(), timestamp.clone().into());

                    if !event.is_empty() {
                        messages.push(event);
                    }
                }

                buffer.clear();
            }

            if let Some(outbox) = outbox.clone() {
                for message in messages {
                    let message = Message::new(message);
                    outbox.send(message).await.unwrap();
                }
            }
        }

        socket.shutdown().await.expect("Failed to shutdown socket");
    }
}
