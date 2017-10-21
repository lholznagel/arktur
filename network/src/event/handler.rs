use std::net::{UdpSocket, SocketAddr};

/// Holds all handler
pub struct EventHandler {
    /// TODO: documentation
    pub ping_handler: fn(SocketAddr, &UdpSocket, &str),
    /// TODO: documentation
    pub pong_handler: fn(SocketAddr, &UdpSocket, &str),
    /// TODO: documentation
    pub register_ack_handler: fn(SocketAddr, &UdpSocket, &str),
    /// TODO: documentation
    pub register_handler: fn(SocketAddr, &UdpSocket, &str)
}

/// Contains all handler for events
impl EventHandler {
    /// Creates a new instance of handlers
    pub fn new() -> Self {
        fn empty(_: SocketAddr, _: &UdpSocket, _: &str) {}

        EventHandler {
            ping_handler: empty,
            pong_handler: empty,
            register_ack_handler: empty,
            register_handler: empty
        }
    }

    /// TODO: documentation
    pub fn set_ping_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
        self.ping_handler = function;
        self
    }

    /// TODO: documentation
    pub fn set_pong_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
        self.pong_handler = function;
        self
    }

    /// Registeres a new handler that fires on `REGISTER_ACK`
    ///
    /// TODO: more docu
    pub fn set_register_ack_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
        self.register_ack_handler = function;
        self
    }

    /// Registeres a new handler that fires on `REGISTER`
    ///
    /// TODO: more docu
    pub fn set_register_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
        self.register_handler = function;
        self
    }
}