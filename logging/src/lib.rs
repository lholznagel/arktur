#![deny(missing_docs)]

//! blockchain_logging
//!
//! Logging crate for the blockchain project
//! Contains multiple macros for outputting to the console

/// Default log implementation
///
/// Simply logs the given string
/// Used by all other log macros
///
/// # Example:
///
/// ``` notest
/// use crate blockchain_logging;
///
/// log!("My super cool log")
/// ```
#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        println!("{}", $msg);
    }
}

/// Logs an debug message
///
/// Color of the output is orange
#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        let mut output = String::from("\x1B[93mDebug   - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

/// Logs an error message
///
/// Color of the output is red
#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        let mut output = String::from("\x1B[31mError   - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}


/// Logs an successful message
///
/// Color of the output is green
#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        let mut output = String::from("\x1B[32mSuccess - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

/// Logs an info message
///
/// Color of the output is blue
#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        let mut output = String::from("\x1B[1;34mInfo    - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

/// Logs an sending message
///
/// Exmaple: A new peer registeres and a ping is send to the peer
///
/// Color of the output is purple
#[macro_export]
macro_rules! sending {
    ($msg:expr) => {
        let mut output = String::from("\x1B[35mSending - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

/// Logs an event message
///
/// Example: A peer got a pong from another peer
///
/// Color of the output is red
#[macro_export]
macro_rules! event {
    ($msg:expr) => {
        let mut output = String::from("\x1B[36mEvent   - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}