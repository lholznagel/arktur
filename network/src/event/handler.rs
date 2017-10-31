use blockchain_protocol::BlockchainProtocol;
use std::net::{UdpSocket, SocketAddr};

/// Holds all handler
pub struct EventHandler {
    /// Function that is called on a PING-event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub ping_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol),

    /// Function is be called on a PONG-event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub pong_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol),

    /// Function that is called on a REGISTER-event
    /// Code: 16
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub register_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol),

    /// Function that is called on a ACK_REGISTER-event
    /// Code: 17
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub register_ack_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol),

    /// Function that is called on a PEER_REGISTERING event
    /// Code: 18
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - parsed protocol
    pub peer_registering_handler: fn(SocketAddr, &UdpSocket, BlockchainProtocol)
}

/// Contains all handler for events
impl EventHandler {
    /// Creates a new instance of handlers
    pub fn new() -> Self {
        fn empty(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {}

        EventHandler {
            ping_handler: empty,
            pong_handler: empty,
            peer_registering_handler: empty,
            register_ack_handler: empty,
            register_handler: empty
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_ping_handler(ping_handler);
    /// # }
    ///
    /// fn ping_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {
    ///     // do something
    /// }
    /// ```
    pub fn set_ping_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_pong_handler(pong_handler);
    /// # }
    ///
    /// fn pong_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {
    ///     // do something
    /// }
    /// ```
    pub fn set_pong_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_pong_handler(peer_registering_handler);
    /// # }
    ///
    /// fn peer_registering_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {
    ///     // do something
    /// }
    /// ```
    pub fn set_peer_registering_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_register_ack_handler(register_ack_handler);
    /// # }
    ///
    /// fn register_ack_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {
    ///     // do something
    /// }
    /// ```
    pub fn set_register_ack_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// # fn main() {
    /// let event_handler = EventHandler::new();
    /// event_handler.set_register_handler(register_handler);
    /// # }
    ///
    /// fn register_handler(_: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {
    ///     // do something
    /// }
    /// ```
    pub fn set_register_handler(mut self, function: fn(SocketAddr, &UdpSocket, BlockchainProtocol)) -> Self {
        self.register_handler = function;
        self
    }
}