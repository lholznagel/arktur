use carina_core::{Config, Event};
use std::net::UdpSocket;

struct A {
    pub answered: Vec<String>
}

impl A {
    pub fn new() -> Self {
        Self{
            answered: Vec::new()
        }
    }
}

pub struct Pong {
    service: A
}

impl Pong {
    pub fn new() -> Self {
        Self {
            service: A::new()
        }
    }
}

impl Event for Pong {
    fn execute(&mut self, _: UdpSocket, source: String, _: &mut Config) {
        info!("[PONG] Received pong event");
        self.service.answered.push(source);
    }
}