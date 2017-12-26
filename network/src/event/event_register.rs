use event::{EmptyPing, PingEvent};

/// Registers all events
pub struct EventRegister {
    /// Function that is called on a PING-event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address from the peer that send the message
    /// - `udpSocket` - connection of the udp socket
    pub ping_handler: Box<PingEvent>
}

impl EventRegister {
    /// Creates new empty handlers
    pub fn new() -> Self {
        EventRegister {
            ping_handler: Box::new(EmptyPing::new())
        }
    }

    /// Registers a `PING` event handler
    ///
    /// For examples please see the trait for `PingEvent`
    ///
    /// # Parameters
    ///
    /// - `Box<PingEvent> - Box that contains the struct that implements the trait `PingEvent`
    pub fn register_ping_handler(mut self, handler: Box<PingEvent>) -> Self {
        self.ping_handler = handler;
        self
    }
}