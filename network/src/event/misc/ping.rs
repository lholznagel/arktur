use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::PingPayload;
use std::boxed::Box;
use std::net::SocketAddr;

trait PingEventClone {
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

/// TODO:
pub trait PingEvent: PingEventClone {
    /// TODO:
    fn handle_event(self: Box<Self>, BlockchainProtocol<PingPayload>, SocketAddr) -> Vec<u8>;
}

/// TODO:
#[derive(Clone)]
pub struct EmptyPing;

impl EmptyPing {
    /// TODO:
    pub fn new() -> Self { EmptyPing }
}

impl PingEvent for EmptyPing {
    fn handle_event(self: Box<Self>, _: BlockchainProtocol<PingPayload>, _: SocketAddr) -> Vec<u8> { vec![0] }
}