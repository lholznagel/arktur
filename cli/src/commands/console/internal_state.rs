use std::collections::HashMap;

pub struct InternalState {
    /// Contains the content for the next block
    pub content: HashMap<String, String>
}

impl InternalState {
    pub fn new() -> Self {
        Self {
            content: HashMap::new()
        }
    }
}