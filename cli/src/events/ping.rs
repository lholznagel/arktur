use carina_core::Event;

pub struct Ping;

impl Event for Ping {
    fn execute(&self) {
        info!("[PING] Received ping event");
    }
}