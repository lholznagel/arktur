use carina_protocol::nacl::Nacl;

use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::PublicKey;

use std::collections::HashMap;

pub struct State {
    // All known peers
    pub peers: HashMap<String, (PublicKey, u8)>, // (public_key, heartbeat)
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