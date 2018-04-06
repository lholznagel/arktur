use carina_protocol::nacl::Nacl;

use std::collections::HashMap;

pub struct State {
    // All known peers
    pub peers: HashMap<String, u8>,
    /// nacl
    pub nacl: Nacl
}

impl State {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
            nacl: Nacl::new()
        }
    }
}