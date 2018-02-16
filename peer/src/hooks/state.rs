use blockchain_protocol::payload::{FoundBlockPayload, Payload};

use std::collections::HashMap;

pub struct State {
    /// Current generated block
    pub current_block: FoundBlockPayload,
    /// Contains the hashes from other peers
    pub hashes: Vec<String>,
    /// is currently a block calculating
    pub is_calculating: bool,
    /// contains the data for the next block
    pub next_block: HashMap<String, String>,
    /// all peers this peer is connected to
    pub peers: Vec<String>
}

impl State {
    pub fn new() -> Self {
        Self {
            current_block: FoundBlockPayload::new(),
            hashes: Vec::new(),
            is_calculating: false,
            next_block: HashMap::new(),
            peers: Vec::new()
        }
    }
}