/// this enum contains all available events
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Events {
    /// Ping event
    /// Event: 0
    Ping,
    /// Pong event
    /// Event: 1
    Pong,
    /// An invalid event
    Invalid
}

/// Converts an number to a enum value
pub fn as_enum(value: u8) -> Events {
    match value {
        0 => Events::Ping,
        1 => Events::Pong,
        _ => Events::Invalid,
    }
}

/// Converts a `Event` to a u8 value
pub fn as_val(value: Events) -> u8 {
    match value {
        Events::Ping => 0,
        Events::Pong => 1,
        _ => 255,
    }
}