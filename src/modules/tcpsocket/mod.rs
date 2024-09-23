use serde::{Deserialize, Serialize};

pub mod tcpsocket;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TCPListenerConfiguration {
    address: String,
    port: u16,
    buffer_size: usize,
}
