use carina_core::Event;
use carina_core::Config;
use std::net::UdpSocket;

pub struct Pong;

impl Event for Pong {
    fn execute(&mut self, _: UdpSocket, source: String, _: &mut Config) {
        info!("[CONSOLE_PONG] Received pong event from {:?}", source);
    }
}