//! Enum for all available events
//!
//! Has some helper to convert an integer to enum value
//! and for converting a enum to an integer value

/// See the fields for documentation
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EventCodes {
    /// This event should be fired to check if another peer
    /// is active.
    ///
    /// Listen to the PONG event for an answer.
    ///
    /// Every peer shall include this event handler.
    ///
    /// Code: 0
    Ping,
    /// Event is fired as a response to a ping event.
    ///
    /// Code: 1
    Pong,
    /// This event should be used to register at a hole puncher.
    ///
    /// Code: 66
    Register,
    /// This event is send by the hole puncher, when the registration
    /// is successful.
    ///
    /// Code: 67
    RegisterAck,
    /// This event is to add data for the next block
    ///
    /// Code: 32
    DataForBlock,
    /// This event is send by the connection manager, when a new block
    /// should be mined
    /// Content of the message contains the index of the block,
    /// the content of the block, the timestamp of the block,
    /// and the hash of the last block.
    /// The expected result is hash and a nonce that was used to generate
    /// this block
    ///
    /// Code: 33
    NewBlock,
    /// This event is fired by a peer that found a possible block
    /// Content of this message should be the nonce and the hash
    ///
    /// Code: 34
    PossibleBlock,
    /// When a possible block is found all peers need to validate it
    ///
    /// Code: 35
    ValidateHash,
    /// Validated hash by the peers
    ///
    /// Code: 36
    ValidatedHash,
    /// Fired by the connection manager, when a block was found
    /// Contains the all information about a block
    /// All peers should stop mining when this message comes
    ///
    /// Code: 37
    FoundBlock,

    /// Notifies a given peer that a connection should be established to another peer
    ///
    /// Code: 48
    HolePuncherConn,

    /// Fired when a peer syncs his peers
    ///
    /// Code: 96
    SyncPeers,
    /// Fired when a peer wants to sync blocks
    ///
    /// Code: 97
    SyncBlocks,
    /// Contains 1000 block hashes
    ///
    /// Code: 98
    SyncBlocksAck,
    /// Requests an block
    ///
    /// Code: 99
    SyncBlocksReq,
    /// Sends back the requestsed block
    ///
    /// Code: 100
    SyncBlocksReqAck,

    /// Fired when the debugger explores the network
    ///
    /// Code: 240
    ExploreNetwork,

    /// Fired when the umber does not match any events
    ///
    /// Code: 255
    NotAValidEvent,
}

/// Converts an integer to the corresponding `EventCode` enum value
///
/// # Parameters
///
/// - `value` - value that should be converted into a enum value
///
/// # Return
///
/// Enum value
///
/// # Example
/// ```
/// use blockchain_hooks::{as_enum, EventCodes};
///
/// match as_enum(0) {
///     EventCodes::Ping => {},
///     _ => panic!("Wrong outcome")
/// }
/// ```
pub fn as_enum(value: u8) -> EventCodes {
    match value {
        0 => EventCodes::Ping,
        1 => EventCodes::Pong,
        64 => EventCodes::Register,
        67 => EventCodes::RegisterAck,
        32 => EventCodes::DataForBlock,
        33 => EventCodes::NewBlock,
        34 => EventCodes::PossibleBlock,
        35 => EventCodes::ValidateHash,
        36 => EventCodes::ValidatedHash,
        37 => EventCodes::FoundBlock,
        48 => EventCodes::HolePuncherConn,
        96 => EventCodes::SyncPeers,
        97 => EventCodes::SyncBlocks,
        98 => EventCodes::SyncBlocksAck,
        99 => EventCodes::SyncBlocksReq,
        100 => EventCodes::SyncBlocksReqAck,
        240 => EventCodes::ExploreNetwork,
        _ => EventCodes::NotAValidEvent,
    }
}

/// Converts a enum value to the corresponding integer value
///
/// # Parameters
///
/// - `value` - value that should be converted to an integer value
///
/// # Return
///
/// Integer value
///
/// # Example
/// ```
/// use blockchain_hooks::{as_number, EventCodes};
///
/// assert_eq!(as_number(EventCodes::Ping), 0);
/// ```
pub fn as_number(value: EventCodes) -> u8 {
    match value {
        EventCodes::Ping => 0,
        EventCodes::Pong => 1,
        EventCodes::Register => 64,
        EventCodes::RegisterAck => 67,
        EventCodes::DataForBlock => 32,
        EventCodes::NewBlock => 33,
        EventCodes::PossibleBlock => 34,
        EventCodes::ValidateHash => 35,
        EventCodes::ValidatedHash => 36,
        EventCodes::FoundBlock => 37,
        EventCodes::HolePuncherConn => 48,
        EventCodes::SyncPeers => 96,
        EventCodes::SyncBlocks => 97,
        EventCodes::SyncBlocksAck => 98,
        EventCodes::SyncBlocksReq => 99,
        EventCodes::SyncBlocksReqAck => 100,
        EventCodes::ExploreNetwork => 240,
        EventCodes::NotAValidEvent => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_enum() {
        match as_enum(0) {
            EventCodes::Ping => {}
            _ => panic!("Wrong outcome"),
        }
    }

    #[test]
    fn get_number() {
        assert_eq!(as_number(EventCodes::Ping), 0);
    }
}