#[macro_export]
macro_rules! log {
    ($msg:expr) => {
        println!("{}", $msg);
    }
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        let mut output = String::from("\x1B[31mError   - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        let mut output = String::from("\x1B[32mSuccess - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        let mut output = String::from("\x1B[1;34mInfo    - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

#[macro_export]
macro_rules! sending {
    ($msg:expr) => {
        let mut output = String::from("\x1B[35mSending - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

#[macro_export]
macro_rules! event {
    ($msg:expr) => {
        let mut output = String::from("\x1B[36mEvent   - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        let mut output = String::from("\x1B[93mDebug   - ");
        output.push_str(&$msg);
        output.push_str("\x1B[0m");
        log!(output);
    }
}