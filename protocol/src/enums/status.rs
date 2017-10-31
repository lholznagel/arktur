//! Enum for all available statuses
//!
//! Has some helper to convert an integer to enum value
//! and for converting a enum to an integer value

/// See the fields for documentation
#[derive(Debug, PartialEq)]
pub enum StatusCodes {
    /// Default status that everything is ok
    ///
    /// Code: 0
    Ok,
    /// Status when the hole puncher has no peer
    ///
    /// Code: 16
    NoPeer,
    /// Undefined status
    ///
    /// Code: 255
    Undefined
}

/// Converts an integer to the corresponding `StatusCode` enum value
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
/// use blockchain_protocol::enums::status::{as_enum, StatusCodes};
///
/// match as_enum(0) {
///     StatusCodes::Ok => {},
///     _ => panic!("Wrong outcome")
/// }
/// ```
pub fn as_enum(value: u8) -> StatusCodes {
    match value {
        0 => StatusCodes::Ok,
        16 => StatusCodes::NoPeer,
        _ =>StatusCodes::Undefined,
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
/// use blockchain_protocol::enums::status::{as_number, StatusCodes};
///
/// assert_eq!(as_number(StatusCodes::Ok), 0);
/// ```
pub fn as_number(value: StatusCodes) -> u8 {
    match value {
        StatusCodes::Ok => 0,
        StatusCodes::NoPeer => 16,
        StatusCodes::Undefined => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_enum() {
        match as_enum(0) {
            StatusCodes::Ok => {},
            _ => panic!("Wrong outcome")
        }
    }

    #[test]
    fn get_number() {
        assert_eq!(as_number(StatusCodes::Ok), 0);
    }
}