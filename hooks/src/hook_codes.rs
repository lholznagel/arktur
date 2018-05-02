//! Enum for all available hooks
//!
//! Has some helper to convert an integer to enum value
//! and for converting a enum to an integer value

/// See the fields for documentation
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HookCodes {
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
    /// Register at a peer or a hole puncher
    ///
    /// Code: 64
    Register,
    /// Acknowledges the registration by sending back a list of peers
    ///
    /// Code: 65
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
    /// Generates a new block
    ///
    /// Code: 133
    BlockGen,
    /// Fired by the connection manager, when a block was found
    /// Contains the all information about a block
    /// All peers should stop mining when this message comes
    ///
    /// Code: 134
    BlockFound,
    /// When a possible block is found all peers need to validate it
    ///
    /// Code: 135
    HashVal,
    /// Validated hash by the peers
    ///
    /// Code: 136
    HashValAck,

    /// Fired when the umber does not match any hooks
    ///
    /// Code: 255
    NotAValidType,
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
/// use carina_hooks::{as_enum, HookCodes};
///
/// match as_enum(0) {
///     HookCodes::Ping => {},
///     _ => panic!("Wrong outcome")
/// }
/// ```
pub fn as_enum(value: u8) -> HookCodes {
    match value {
        0 => HookCodes::Ping,
        1 => HookCodes::Pong,
        64 => HookCodes::Register,
        65 => HookCodes::RegisterAck,
        128 => HookCodes::GetBlocks,
        129 => HookCodes::GetBlocksAck,
        130 => HookCodes::GetBlock,
        131 => HookCodes::GetBlockAck,
        132 => HookCodes::BlockData,
        133 => HookCodes::BlockGen,
        134 => HookCodes::BlockFound,
        135 => HookCodes::HashVal,
        136 => HookCodes::HashValAck,
        _ => HookCodes::NotAValidType,
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
/// use carina_hooks::{as_number, HookCodes};
///
/// assert_eq!(as_number(HookCodes::Ping), 0);
/// ```
pub fn as_number(value: HookCodes) -> u8 {
    match value {
        HookCodes::Ping => 0,
        HookCodes::Pong => 1,
        HookCodes::Register => 64,
        HookCodes::RegisterAck => 65,
        HookCodes::GetBlocks => 128,
        HookCodes::GetBlocksAck => 129,
        HookCodes::GetBlock => 130,
        HookCodes::GetBlockAck => 131,
        HookCodes::BlockData => 132,
        HookCodes::BlockGen => 133,
        HookCodes::BlockFound => 134,
        HookCodes::HashVal => 135,
        HookCodes::HashValAck => 136,
        HookCodes::NotAValidType => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_enum() {
        match as_enum(0) {
            HookCodes::Ping => {}
            _ => panic!("Wrong outcome"),
        }
    }

    #[test]
    fn get_number() {
        assert_eq!(as_number(HookCodes::Ping), 0);
    }
}