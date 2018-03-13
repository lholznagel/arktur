use blockchain_protocol::payload::Payload;
use blockchain_protocol::payload::blocks::BlockFound;

use std::collections::HashMap;

pub struct State {
    /// Current generated block
    pub current_block: BlockFound,
    /// Contains the hashes from other peers
    pub hashes: Vec<String>,
    /// is currently a block calculating
    pub is_calculating: bool,
    /// contains the data for the next block
    pub next_block: HashMap<String, String>,
    /// all peers this peer is connected to
    pub peers: HashMap<String, u8>,
    /// location for all blocks
    pub storage: String
}

impl State {
    pub fn new(storage: String) -> Self {
        Self {
            current_block: BlockFound::new(),
            hashes: Vec::new(),
            is_calculating: false,
            next_block: HashMap::new(),
            peers: HashMap::new(),
            storage
        }
    }
}