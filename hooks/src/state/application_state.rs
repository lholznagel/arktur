use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

/// Struct that represents the global state
#[derive(Debug)]
pub struct ApplicationState<T> {
    /// Buffer of the message that is send
    pub payload_buffer: Vec<u8>,
    /// Source address
    pub source: String,
    /// State for the application
    pub state: Arc<Mutex<T>>,
    /// Open UDP connection
    pub udp: UdpSocket,
}