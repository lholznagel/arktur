use carina_protocol::payload::Payload;
use carina_protocol::payload::blocks::BlockFound;

use sodiumoxide::crypto::box_;

use std::collections::HashMap;

#[derive(Clone, Debug)]
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
    pub storage: String,
    /// public nacl key
    pub pub_key: [u8; 32],
    /// private nacl key
    pub priv_key: [u8; 32]
}

impl State {
    pub fn new(storage: String) -> Self {
        let (pub_key, priv_key) = box_::gen_keypair();

        Self {
            current_block: BlockFound::new(),
            hashes: Vec::new(),
            is_calculating: false,
            next_block: HashMap::new(),
            peers: HashMap::new(),
            storage,
            pub_key: pub_key.0,
            priv_key: priv_key.0
        }
    }
}