use carina_core::{Config, Event};
use std::net::UdpSocket;

pub struct Pong {
    pub answered: Vec<String>
}

impl Pong {
    pub fn new() -> Self {
        Self{
            answered: Vec::new()
        }
    }
}

impl Event for Pong {
    fn execute(&mut self, _: UdpSocket, source: String, _: &mut Config) {
        info!("[PONG] Received pong event");
        self.answered.push(source);
    }
}