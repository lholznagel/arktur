use carina_protocol::nacl::Nacl;
use carina_protocol::payload::blocks::BlockFound;
use carina_protocol::payload::Payload;
use config::Config;

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
    pub peers: HashMap<String, (PublicKey, u8, bool)>, // (public_key, heartbeat)
    /// location for all blocks
    pub storage: String,
    /// nacl public and secret key
    pub nacl: Nacl
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut peers = HashMap::new();
        for peer in config.peers {
            peers.insert(peer.address.clone(), (peer.public_key(), 0, false));
        }

        Self {
            current_block: BlockFound::new(),
            hashes: Vec::new(),
            is_calculating: false,
            next_block: HashMap::new(),
            peers,
            storage: config.storage,
            nacl: Nacl::new()
        }
    }
}