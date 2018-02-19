pub struct State {
    // All known peers
    pub peers: Vec<String>
}

impl State {
    pub fn new() -> Self {
        Self {
            peers: Vec::new()
        }
    }
}