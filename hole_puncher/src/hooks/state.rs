use std::collections::HashMap;

pub struct State {
    // All known peers
    pub peers: HashMap<String, u8>
}

impl State {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new()
        }
    }
}