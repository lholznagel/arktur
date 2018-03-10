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
    /// Requests a list of peers
    ///
    /// Code: 67
    GetPeers,
    /// Gets a list of peers
    ///
    /// Code: 67
    GetPeersAck,
    /// Register at a peer or a hole puncher
    ///
    /// Code: 66
    Register,
    /// Acknowledges the registration by sending back a list of peers
    ///
    /// Code: 67
    RegisterAck,
    /// Requests a list of blocks
    ///
    /// Code: 128
    GetBlocks,
    /// Gets a response with blocks containing per default 1000 blocks
    ///
    /// Code: 129
    GetBlocksAck,
    /// Requests a specific block
    ///
    /// Code: 130
    GetBlock,
    /// Gets the requested block
    ///
    /// Code: 131
    GetBlockAck,
    /// Adds new data ro the next block
    ///
    /// Code: 132
    BlockData,

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
        64 => EventCodes::GetPeers,
        65 => EventCodes::GetPeersAck,
        66 => EventCodes::Register,
        67 => EventCodes::RegisterAck,
        128 => EventCodes::GetBlocks,
        129 => EventCodes::GetBlocksAck,
        130 => EventCodes::GetBlock,
        131 => EventCodes::GetBlockAck,
        132 => EventCodes::BlockData,

        33 => EventCodes::NewBlock,
        34 => EventCodes::PossibleBlock,
        35 => EventCodes::ValidateHash,
        36 => EventCodes::ValidatedHash,
        37 => EventCodes::FoundBlock,
        48 => EventCodes::HolePuncherConn,
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
        EventCodes::GetPeers => 64,
        EventCodes::GetPeersAck => 65,
        EventCodes::Register => 66,
        EventCodes::RegisterAck => 67,
        EventCodes::GetBlocks => 128,
        EventCodes::GetBlocksAck => 129,
        EventCodes::GetBlock => 130,
        EventCodes::GetBlockAck => 131,
        EventCodes::BlockData => 132,

        EventCodes::NewBlock => 33,
        EventCodes::PossibleBlock => 34,
        EventCodes::ValidateHash => 35,
        EventCodes::ValidatedHash => 36,
        EventCodes::FoundBlock => 37,
        EventCodes::HolePuncherConn => 48,
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