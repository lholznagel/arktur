use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{PublicKey, SecretKey};

/// Configuration for the peer
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// List of all peers to connect to
    pub peers: Vec<Peer>,
    /// Port the peer should listen on
    pub port: u16,
    /// Storage for the blocks
    pub storage: String,
    /// Pirvate key
    secret_key: SecretKey
}

/// Peer configuration
#[derive(Clone, Debug, PartialEq)]
pub struct Peer {
    /// Address of the peer. Example: 127.0.0.1:4500
    pub address: String,
    /// Public Key of the peer
    pub public_key: PublicKey,
}

impl Config {
    /// Creates a new config instance
    pub fn new(port: u16, peers: Vec<Peer>, storage: String, secret_key: SecretKey) -> Self {
        Self {
            port,
            peers,
            storage,
            secret_key
        }
    }
}