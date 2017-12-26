use blockchain_protocol::payload::PingPayload;
use event::{EmptyPing, PingEvent};

/// TODO:
pub struct EventRegister {
    /// TODO:
    pub ping_handler: Box<PingEvent>
}

impl EventRegister {
    /// TODO:
    pub fn new() -> Self {
        EventRegister {
            ping_handler: Box::new(EmptyPing::new())
        }
    }

    /// TODO:
    pub fn register_ping_handler(mut self, handler: Box<PingEvent>) -> Self {
        self.ping_handler = handler;
        self.ping_handler.clone();
        self
    }
}