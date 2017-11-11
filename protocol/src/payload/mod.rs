//! Module for all payload models
mod payload_parser;
mod ping;
mod pong;
mod register;
mod register_ack;
mod peer_registering;
mod new_block;
mod possible_block;
mod found_block;

pub use self::payload_parser::PayloadModel;
pub use self::ping::PingPayload;
pub use self::pong::PongPayload;
pub use self::register::RegisterPayload;
pub use self::register_ack::RegisterAckPayload;
pub use self::peer_registering::PeerRegisteringPayload;
pub use self::new_block::NewBlockPayload;
pub use self::possible_block::PossibleBlockPayload;
pub use self::found_block::FoundBlockPayload;