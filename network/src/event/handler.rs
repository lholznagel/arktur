use std::net::SocketAddr;

/// Holds all handler
pub struct EventHandler {
    /// TODO: documentation
    pub register_handler: fn(SocketAddr, String),
    /// TODO: documentation
    pub register_ack_handler: fn(SocketAddr, String)
}

/// Contains all handler for events
impl EventHandler {
    /// Creates a new instance of handlers
    pub fn new() -> Self {
        fn empty(_: SocketAddr, _: String) {
            println!("DEFAULT");
        }

        EventHandler {
            register_handler: empty,
            register_ack_handler: empty
        }
    }

    /// Registeres a new handler that fires on `REGISTER`
    ///
    /// TODO: more docu
    pub fn set_register_handler(mut self, function: fn(SocketAddr, String)) -> Self {
        self.register_handler = function;
        self
    }

    /// TODO: documentation
    pub fn register_handler(self) -> fn(SocketAddr, String) {
        self.register_handler
    }

    /// Registeres a new handler that fires on `REGISTER_ACK`
    ///
    /// TODO: more docu
    pub fn set_register_ack_handler(mut self, function: fn(SocketAddr, String)) -> Self {
        self.register_ack_handler = function;
        self
    }
}