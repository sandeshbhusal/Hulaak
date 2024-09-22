use serde::{Deserialize, Serialize};

pub mod udpsocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UdpSocketConfiguration {
    address: String,
    port: u16,
    buffer_size: usize,
}
