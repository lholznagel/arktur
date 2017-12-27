use event::*;

/// Registers all events
pub struct EventRegister {
    /// Contains the struct that is used on a ping event
    /// Code: 0
    pub ping_handler: Box<PingEvent>,

    /// Contains the struct that is used on a pong event
    /// Code: 1
    pub pong_handler: Box<PongEvent>
}

impl EventRegister {
    /// Creates new empty handlers
    pub fn new() -> Self {
        EventRegister {
            ping_handler: Box::new(EmptyPing::new()),
            pong_handler: Box::new(EmptyPong::new())
        }
    }

    /// Registers a `PING` event handler
    ///
    /// For examples please see the `PingEvent` trait
    ///
    /// # Parameters
    ///
    /// - `Box<PingEvent> - Box that contains the struct that implements the trait `PingEvent`
    pub fn register_ping_handler(mut self, handler: Box<PingEvent>) -> Self {
        self.ping_handler = handler;
        self
    }

    /// Registers a `PONG` event handler
    ///
    /// For examples please see the `PongEvent` trait
    ///
    /// # Parameters
    ///
    /// - `Box<PongEvent> - Box that contains the struct that implements the trait `PongEvent`
    pub fn register_pong_handler(mut self, handler: Box<PongEvent>) -> Self {
        self.pong_handler = handler;
        self
    }
}