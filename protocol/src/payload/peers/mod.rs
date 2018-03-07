//! Contains all payload parser that handle registering
mod get_peers_ack;
mod register_ack;

pub use self::get_peers_ack::GetPeersAckPayload;
pub use self::register_ack::RegisterAckPayload;