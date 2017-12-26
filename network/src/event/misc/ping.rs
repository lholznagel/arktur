use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::PingPayload;
use std::boxed::Box;
use std::net::SocketAddr;

/// Trait for implementing the Ping event
///
/// # Example
///
/// ```
/// extern crate blockchain_protocol;
/// extern crate blockchain_network;
///
/// use blockchain_protocol::BlockchainProtocol;
/// use blockchain_protocol::payload::PingPayload;
/// use blockchain_network::event::EventRegister;
/// use blockchain_network::event::PingEvent;
/// use std::boxed::Box;
/// use std::net::SocketAddr;
///
/// #[derive(Clone)]
/// pub struct Handler;
///
/// impl PingEvent for Handler {
///     fn handle_event(self: Box<Self>, message: BlockchainProtocol<PingPayload>, source: SocketAddr) -> Vec<u8> {
///         // do something
///         vec![0]
///     }
/// }
///
/// fn main() {
///     let register = EventRegister::new();
///     register.register_ping_handler(Box::new(Handler{}));
/// }
/// ```
pub trait PingEvent: PingEventClone {
    /// Function that is executed when the event is fired
    ///
    /// For examples see the comments at `event_register.rs`
    ///
    /// # Params
    ///
    /// - `BlockchainProtocol<PingPayload>` - contains the parsed protocol
    /// - `SocketAddr` - source address
    ///
    /// # Return
    ///
    /// - `Vec<u8>` - Vector that should be returned to the source address
    fn handle_event(self: Box<Self>, BlockchainProtocol<PingPayload>, SocketAddr) -> Vec<u8>;
}

/// Trait for implementing clone for the PingEvent
pub trait PingEventClone {
    fn clone_box(&self) -> Box<PingEvent>;
}

impl<T> PingEventClone for T where T: 'static + PingEvent + Clone {
    fn clone_box(&self) -> Box<PingEvent> {
        Box::new(self.clone())
    }
}

impl Clone for Box<PingEvent> {
    fn clone(&self) -> Box<PingEvent> {
        self.clone_box()
    }
}

/// Empty Ping struct. Is the default when creating a new EventRegister
#[derive(Clone)]
pub struct EmptyPing;

impl EmptyPing {
    /// Creates a new EmptyPing instance
    pub fn new() -> Self { EmptyPing }
}

impl PingEvent for EmptyPing {
    fn handle_event(self: Box<Self>, _: BlockchainProtocol<PingPayload>, _: SocketAddr) -> Vec<u8> { vec![0] }
}