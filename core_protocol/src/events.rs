/// this enum contains all available events
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Events {
    /// Event: 0
    Ping,
    /// Event: 1
    Pong,
    /// Event: 64
    NewBlockContent,
    /// An invalid event
    Invalid
}

impl Events {
    /// Converts the given enum value to a number
    pub fn as_val(event: Events) -> u8 {
        match event {
            Events::Ping            => 0,
            Events::Pong            => 1,
            Events::NewBlockContent => 64,
            _                       => 255
        }
    }

    /// Converts the given value to the enum value
    pub fn as_enum(value: u8) -> Events {
        match value {
            0  => Events::Ping,
            1  => Events::Pong,
            64 => Events::NewBlockContent,
            _  => Events::Invalid
        }
    }
}