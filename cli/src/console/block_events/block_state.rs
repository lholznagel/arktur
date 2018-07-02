use std::collections::HashMap;

pub struct BlockState {
    /// Contains the content for the next block
    pub content: HashMap<String, String>
}

impl BlockState {
    pub fn new() -> Self {
        Self {
            content: HashMap::new()
        }
    }

    pub fn reset(&mut self) {
        self.content = HashMap::new();
    }
}