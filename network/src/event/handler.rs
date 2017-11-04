use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::*;
use std::net::{UdpSocket, SocketAddr};

/// Holds all handler
pub struct EventHandler {
    /// Function that is called on a PING-event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address from the peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub ping_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol<PingPayload>),

    /// Function is be called on a PONG-event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address from the peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub pong_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol<PongPayload>),

    /// Function that is called on a REGISTER-event
    /// Code: 16
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address from the peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub register_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol<RegisterPayload>),

    /// Function that is called on a ACK_REGISTER-event
    /// Code: 17
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address from the peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub register_ack_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol<RegisterAckPayload>),

    /// Function that is called on a PEER_REGISTERING event
    /// Code: 18
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address from the peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub peer_registering_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol<PeerRegisteringPayload>)
}

/// Contains all handler for events
impl EventHandler {
    /// Creates a new instance of handlers
    pub fn new() -> Self {
        fn empty_ping(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PingPayload>) {}
        fn empty_pong(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PongPayload>) {}
        fn empty_register(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<RegisterPayload>) {}
        fn empty_register_ack(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<RegisterAckPayload>) {}
        fn empty_peer_registering(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PeerRegisteringPayload>) {}

        EventHandler {
            ping_handler: empty_ping,
            pong_handler: empty_pong,
            register_handler: empty_register,
            register_ack_handler: empty_register_ack,
            peer_registering_handler: empty_peer_registering
        }
    }

    /// Sets the `PING` event handler
    ///
    /// # Parameters
    ///
    /// - `function` - function that should be called
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_network;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::PingPayload;
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_ping_handler(ping_handler);
    /// # }
    ///
    /// fn ping_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PingPayload>) {
    ///     // do something
    /// }
    /// ```
    pub fn set_ping_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol<PingPayload>)) -> Self {
        self.ping_handler = function;
        self
    }

    /// Sets the `PONG` event handler
    ///
    /// # Parameters
    ///
    /// - `function` - function that should be called
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_network;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::PongPayload;
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_pong_handler(pong_handler);
    /// # }
    ///
    /// fn pong_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PongPayload>) {
    ///     // do something
    /// }
    /// ```
    pub fn set_pong_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol<PongPayload>)) -> Self {
        self.pong_handler = function;
        self
    }

    /// Sets the `PEER_REGISTERING` event handler
    ///
    /// # Parameters
    ///
    /// - `function` - function that should be called
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_network;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::PeerRegisteringPayload;
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_peer_registering_handler(peer_registering_handler);
    /// # }
    ///
    /// fn peer_registering_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PeerRegisteringPayload>) {
    ///     // do something
    /// }
    /// ```
    pub fn set_peer_registering_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol<PeerRegisteringPayload>)) -> Self {
        self.peer_registering_handler = function;
        self
    }

    /// Sets the `ACK_REGISTER` event handler
    ///
    /// # Parameters
    ///
    /// - `function` - function that should be called
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_network;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::RegisterAckPayload;
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_register_ack_handler(register_ack_handler);
    /// # }
    ///
    /// fn register_ack_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<RegisterAckPayload>) {
    ///     // do something
    /// }
    /// ```
    pub fn set_register_ack_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol<RegisterAckPayload>)) -> Self {
        self.register_ack_handler = function;
        self
    }

    /// Sets the `REGISTER` event handler
    ///
    /// # Parameters
    ///
    /// - `function` - function that should be called
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_network;
    /// extern crate blockchain_protocol;
    ///
    /// use blockchain_protocol::BlockchainProtocol;
    /// use blockchain_protocol::payload::RegisterPayload;
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_register_handler(register_handler);
    /// # }
    ///
    /// fn register_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<RegisterPayload>) {
    ///     // do something
    /// }
    /// ```
    pub fn set_register_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol<RegisterPayload>)) -> Self {
        self.register_handler = function;
        self
    }
}