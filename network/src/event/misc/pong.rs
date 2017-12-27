use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::PongPayload;
use std::boxed::Box;
use std::net::SocketAddr;

/// Trait for implementing the Pong event
///
/// # Example
///
/// ```
/// extern crate blockchain_protocol;
/// extern crate blockchain_network;
///
/// use blockchain_protocol::BlockchainProtocol;
/// use blockchain_protocol::payload::PongPayload;
/// use blockchain_network::event::EventRegister;
/// use blockchain_network::event::PongEvent;
/// use std::boxed::Box;
/// use std::net::SocketAddr;
///
/// #[derive(Clone)]
/// pub struct Handler;
///
/// impl PongEvent for Handler {
///     fn handle_event(self: Box<Self>, message: BlockchainProtocol<PongPayload>, source: SocketAddr) -> Vec<u8> {
///         // do something
///         vec![0]
///     }
/// }
///
/// fn main() {
///     let register = EventRegister::new();
///     register.register_pong_handler(Box::new(Handler{}));
/// }
/// ```
pub trait PongEvent: PongEventClone {
    /// Function that is executed when the event is fired
    ///
    /// For examples see the comments at `event_register.rs`
    ///
    /// # Params
    ///
    /// - `BlockchainProtocol<PongPayload>` - contains the parsed protocol
    /// - `SocketAddr` - source address
    ///
    /// # Return
    ///
    /// - `Vec<u8>` - Vector that should be returned to the source address
    fn handle_event(self: Box<Self>, BlockchainProtocol<PongPayload>, SocketAddr) -> Vec<u8>;
}

/// Trait for implementing clone for the PongEvent
pub trait PongEventClone {
    fn clone_box(&self) -> Box<PongEvent>;
}

impl<T> PongEventClone for T where T: 'static + PongEvent + Clone {
    fn clone_box(&self) -> Box<PongEvent> {
        Box::new(self.clone())
    }
}

impl Clone for Box<PongEvent> {
    fn clone(&self) -> Box<PongEvent> {
        self.clone_box()
    }
}

/// Empty Pong struct. Is the default when creating a new EventRegister
#[derive(Clone)]
pub struct EmptyPong;

impl EmptyPong {
    /// Creates a new EmptyPong instance
    pub fn new() -> Self { EmptyPong }
}

impl PongEvent for EmptyPong {
    fn handle_event(self: Box<Self>, _: BlockchainProtocol<PongPayload>, _: SocketAddr) -> Vec<u8> { vec![0] }
}