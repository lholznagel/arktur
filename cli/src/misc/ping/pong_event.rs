use carina_core::{Config, Event, Peer};
use std::net::UdpSocket;
use std::collections::HashMap;

pub struct Pong {
    pub answered: HashMap<String, bool>
}

impl Pong {
    pub fn new(peers: HashMap<String, Peer>) -> Self {
        let mut answered = HashMap::new();
        for (key, _) in peers {
            answered.insert(key, false);
        }

        Self {
            answered
        }
    }
}

impl Event for Pong {
    fn execute(&mut self, _: UdpSocket, source: String, _: &mut Config, _: &[u8]) {
        debug!("[MISC_PONG] Received pong event");
        self.answered.insert(source, true);
    }
}