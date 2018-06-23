use config::Config;
use std::net::UdpSocket;

/// Trait that every event handler must implement
pub trait Event: Sync + Send {
    /// Called when a message comes in
    fn execute(&mut self, udp: UdpSocket, source: String, config: &mut Config, buffer: &[u8]);
}