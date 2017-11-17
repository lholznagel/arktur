//! handles all files that have to do with peers
mod peer;
mod known_peers;

pub use self::peer::Peer;
pub use self::known_peers::KnownPeers;