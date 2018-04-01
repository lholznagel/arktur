extern crate log;

use log::{Log, Level, Metadata, Record, SetLoggerError};

struct Logger {
    level: Level,
    exclude: Vec<String>
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mut result = true;

        for value in self.exclude.clone() {
            if metadata.target().contains(&format!("{}:", value)) {
                result = false;
            }
        }

        !self.exclude.contains(&metadata.target().to_string()) && result && metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
           return;
        }

        let mut level_msg = String::new();
        match record.level() {
            Level::Error => level_msg.push_str("\x1B[31mErr  "),
            Level::Warn  => level_msg.push_str("\x1B[93mWarn "),
            Level::Info  => level_msg.push_str("\x1B[34mInfo "),
            Level::Debug => level_msg.push_str("\x1B[35mDebug"),
            Level::Trace => level_msg.push_str("\x1B[35mTrace")
        };

        println!("{} - {}\x1B[0m", level_msg, record.args());
    }

    fn flush(&self) {
    }
}

pub struct LogBuilder {
    level: Level,
    exclude: Vec<String>
}

impl LogBuilder {
    pub fn new() -> Self {
        Self {
            level: Level::Info,
            exclude: Vec::new()
        }
    }

    pub fn add_exclude(mut self, name: String) -> Self {
        self.exclude.push(name);
        self
    }

    pub fn set_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn build(self) -> Result<(), SetLoggerError> {
        let logger = Logger { 
            level: self.level, 
            exclude: self.exclude 
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(self.level.to_level_filter());
        Ok(())
    }
}