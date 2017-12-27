use std::net::UdpSocket;

/// TODO:
pub trait Hooks {
    /// TODO:
    fn on_ping(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_pong(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_register(&self, udp: &UdpSocket, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_ack_register(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_peer_registering(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_new_block(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_possible_block(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// TODO:
    fn on_found_block(&self, message: Vec<u8>, source: String) -> Vec<u8>;
}