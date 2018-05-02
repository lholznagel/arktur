use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

/// Every function that handles hooks gets the MessageState injected
/// 
/// The MessageState is Generic over the type T, where T should
/// represent the global state of the application.
/// With this the function has access to the global state and all
/// necessary information about a message.
#[derive(Debug)]
pub struct MessageState<T> {
    /// Buffer of the message that is send
    /// Needs to be parsed by the receiver
    pub payload_buffer: Vec<u8>,
    /// Source address of the message
    pub source: String,
    /// Global state access
    pub state: Arc<Mutex<T>>,
    /// Open UDP connection
    /// Used so that the function can send message over UDP
    pub udp: UdpSocket,
}