//! Struct of the protocol
//!
//! ```
//! // 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 
//! //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! //| Version               | Event                 |
//! //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
//! //|                                               |
//! ////                                             //
//! ////                Payload                      //
//! ////                                             //
//! //|                                               |
//! //+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//! ```
//! 
//! `Version`: Version of the protocol. For now it will always be 1
//! 
//! `Event`: Payload event. For example `ping` has the event code 0
//! 
//! `Payload`: Payload of the request
mod empty;
mod payload;

/// Contains payloads that have to do with blocks
pub mod block;

pub use self::empty::EmptyPayload;
pub use self::payload::Payload;