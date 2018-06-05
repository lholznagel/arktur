use carina_core::Event;

pub struct Pong;

impl Event for Pong {
    fn execute(&self) {
        info!("[PONG] Received pong event");
    }
}