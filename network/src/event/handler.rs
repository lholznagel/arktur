/// Holds all handler
#[derive(Copy, Clone, Debug)]
pub struct EventHandler {
    /// TODO: documentation
    pub register_handler: fn(String),
    /// TODO: documentation
    pub register_ack_handler: fn(String)
}

/// Contains all handler for events
impl EventHandler {
    /// Creates a new instance of handlers
    pub fn new() -> Self {
        fn empty(_: String) {}

        EventHandler {
            register_handler: empty,
            register_ack_handler: empty
        }
    }

    /// Registeres a new handler that fires on `REGISTER`
    ///
    /// TODO: more docu
    pub fn set_register_handler(mut self, function: fn(String)) -> Self {
        self.register_handler = function;
        self
    }

    /// Registeres a new handler that fires on `REGISTER_ACK`
    ///
    /// TODO: more docu
    pub fn set_register_ack_handler(mut self, function: fn(String)) -> Self {
        self.register_ack_handler = function;
        self
    }
}