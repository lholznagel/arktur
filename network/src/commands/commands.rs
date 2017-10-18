/// Contains all commands that are needed
#[allow(non_camel_case_types)]
pub enum Commands {
    /// Registers the sending peer
    REGISTER,
    /// Acknowledge message for registering
    ACK_REGISTER,
}

/// Converts a command into an int value
///
/// # Parameters
///
/// `command` - that should be converted into an int
///
/// # Returns
///
/// Int value from the command
///
/// # Example
///
/// ```
/// use blockchain_network::commands::{Commands, command_as_int};
///
/// let command_int = command_as_int(Commands::REGISTER);
/// assert_eq!(command_int, 0);
/// ```
pub fn command_as_int(command: Commands) -> u8 {
    match command {
        Commands::REGISTER => 0,
        Commands::ACK_REGISTER => 1,
    }
}

/// Converts a int value to the matching command
///
/// # Parameters
///
/// `value` - command as int
///
/// # Returns
///
/// Result containing the Command enum value or an error message
///
/// # Example
///
/// ```
/// use blockchain_network::commands::{Commands, int_as_command};
///
/// // should return Commands::ACK_REGISTER
/// let command_enum = int_as_command(1).unwrap();
/// ```
pub fn int_as_command(value: u8) -> Result<Commands, String> {
    match value {
        0 => Ok(Commands::REGISTER),
        1 => Ok(Commands::ACK_REGISTER),
        _ => Err(String::from("Value is not valid")),
    }
}

#[cfg(test)]
mod tests {
    use super::{Commands, command_as_int, int_as_command};

    #[test]
    fn get_command_as_int() {
        assert_eq!(command_as_int(Commands::REGISTER), 0);
        assert_eq!(command_as_int(Commands::ACK_REGISTER), 1);
    }

    #[test]
    fn get_int_as_command() {
        let result = int_as_command(0).unwrap();
        match result {
            Commands::REGISTER => {}
            _ => panic!("Result is wrong"),
        };

        let result = int_as_command(1).unwrap();
        match result {
            Commands::ACK_REGISTER => {}
            _ => panic!("Result is wrong"),
        };
    }
}