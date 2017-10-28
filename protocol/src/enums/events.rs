//! Enum for all available events
//!
//! Has some helper to convert an integer to enum value
//! and for converting a enum to an integer value

/// See the fields for documentation
#[derive(Debug, PartialEq)]
pub enum EventCodes {
    /// This event shoudl be fired to check if another peer
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
    /// Code: 16
    Register,
    /// This event is send by the hole puncher, when  the registration
    /// is successful.
    /// The body of the message contains the IP and Port of another peer.
    /// The peer shall send a PING event to the given peer in order
    /// to open a connection.
    ///
    /// Code: 17
    AckRegister,
    /// This event is send by the hole puncher, when a new
    /// peer registers itself.
    /// Content of this message includes the IP, and Port of
    /// the new peer.
    /// The peer shall send a PING event to the new peer
    /// in order to open a connection.
    ///
    /// Code: 18
    PeerRegistering,
    /// Fired when the umber does not match any events
    ///
    /// Code: 255
    NotAValidEvent
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
/// use blockchain_protocol::enums::events::{as_enum, EventCodes};
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
        16 => EventCodes::Register,
        17 => EventCodes::AckRegister,
        18 => EventCodes::PeerRegistering,
        _ =>EventCodes::NotAValidEvent,
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
/// use blockchain_protocol::enums::events::{as_number, EventCodes};
///
/// assert_eq!(as_number(EventCodes::Ping), 0);
/// ```
pub fn as_number(value: EventCodes) -> u8 {
    match value {
        EventCodes::Ping => 0,
        EventCodes::Pong => 1,
        EventCodes::Register => 16,
        EventCodes::AckRegister => 17,
        EventCodes::PeerRegistering => 18,
        EventCodes::NotAValidEvent => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_enum() {
        match as_enum(0) {
            EventCodes::Ping => {},
            _ => panic!("Wrong outcome")
        }
    }

    #[test]
    fn get_number() {
        assert_eq!(as_number(EventCodes::Ping), 0);
    }
}