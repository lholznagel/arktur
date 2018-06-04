use carina_core::Event;

pub struct Ping;

impl Event for Ping {
    fn execute(&self) {
        info!("[PING] Received ping event");
    }
}

pub struct Ping2;

impl Event for Ping2 {
    fn execute(&self) {
        info!("[PING2] Received ping event");
    }
}

pub struct Ping3;

impl Event for Ping3 {
    fn execute(&self) {
        info!("[PING3] Received ping event");
    }
}