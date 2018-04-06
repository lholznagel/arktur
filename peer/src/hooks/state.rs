use carina_protocol::nacl::Nacl;
use carina_protocol::payload::Payload;
use carina_protocol::payload::blocks::BlockFound;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;

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
    pub peers: HashMap<String, (PublicKey, u8)>, // (public_key, heartbeat)
    /// location for all blocks
    pub storage: String,
    /// nacl public and secret key
    pub nacl: Nacl
}

impl State {
    pub fn new(storage: String) -> Self {
        Self {
            current_block: BlockFound::new(),
            hashes: Vec::new(),
            is_calculating: false,
            next_block: HashMap::new(),
            peers: HashMap::new(),
            storage,
            nacl: Nacl::new()
        }
    }
}