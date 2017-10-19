/// Contains all events that are needed
#[allow(non_camel_case_types)]
pub enum Events {
    /// Registers the sending peer
    REGISTER,
    /// Acknowledge message for registering
    ACK_REGISTER,
}

/// Converts a event into an int value
///
/// # Parameters
///
/// `event` - that should be converted into an int
///
/// # Returns
///
/// Int value from the event
///
/// # Example
///
/// ```
/// use blockchain_network::event::{Events, event_as_int};
///
/// let event_int = event_as_int(Events::REGISTER);
/// assert_eq!(event_int, 0);
/// ```
pub fn event_as_int(event: Events) -> u8 {
    match event {
        Events::REGISTER => 0,
        Events::ACK_REGISTER => 1,
    }
}

/// Converts a int value to the matching event
///
/// # Parameters
///
/// `value` - event as int
///
/// # Returns
///
/// Result containing the Event enum value or an error message
///
/// # Example
///
/// ```
/// use blockchain_network::event::{Events, int_as_event};
///
/// // should return Events::ACK_REGISTER
/// let event_enum = int_as_event(1).unwrap();
/// ```
pub fn int_as_event(value: u8) -> Result<Events, String> {
    match value {
        0 => Ok(Events::REGISTER),
        1 => Ok(Events::ACK_REGISTER),
        _ => Err(String::from("Value is not valid")),
    }
}

#[cfg(test)]
mod tests {
    use super::{Events, event_as_int, int_as_event};

    #[test]
    fn get_event_as_int() {
        assert_eq!(event_as_int(Events::REGISTER), 0);
        assert_eq!(event_as_int(Events::ACK_REGISTER), 1);
    }

    #[test]
    fn get_int_as_event() {
        let result = int_as_event(0).unwrap();
        match result {
            Events::REGISTER => {}
            _ => panic!("Result is wrong"),
        };

        let result = int_as_event(1).unwrap();
        match result {
            Events::ACK_REGISTER => {}
            _ => panic!("Result is wrong"),
        };
    }
}