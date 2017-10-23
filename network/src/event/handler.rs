use std::net::{UdpSocket, SocketAddr};

/// Holds all handler
pub struct EventHandler {
    /// Function that is called on a PING-event
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - message that was send with the event
    pub ping_handler: fn(SocketAddr, &UdpSocket, &str),

    /// Function is be called on a PONG-event
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - message that was send with the event
    pub pong_handler: fn(SocketAddr, &UdpSocket, &str),

    /// Function that is called on a PEER_REGISTERING event
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - message that was send with the event
    pub peer_registering_handler: fn(SocketAddr, &UdpSocket, &str),

    /// Function that is called on a ACK_REGISTER-event
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - message that was send with the event
    pub register_ack_handler: fn(SocketAddr, &UdpSocket, &str),

    /// Function that is called on a REGISTER-event
    ///
    /// # Parameters
    ///
    /// - `socketAddr` - socket address fromt he peer that send the message
    /// - `udpSocket` - connection of the udp socket
    /// - `message` - message that was send with the event
    pub register_handler: fn(SocketAddr, &UdpSocket, &str)
}

/// Contains all handler for events
impl EventHandler {
    /// Creates a new instance of handlers
    pub fn new() -> Self {
        fn empty(_: SocketAddr, _: &UdpSocket, _: &str) {}

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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// fn ping_handler(_: SocketAddr, _: &UdpSocket, _: &str) {
    ///     // do something
    /// }
    ///
    /// let event_handler = EventHandler::new();
    /// event_handler.set_ping_handler(ping_handler);
    /// ```
    pub fn set_ping_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// fn pong_handler(_: SocketAddr, _: &UdpSocket, _: &str) {
    ///     // do something
    /// }
    ///
    /// let event_handler = EventHandler::new();
    /// event_handler.set_pong_handler(pong_handler);
    /// ```
    pub fn set_pong_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// fn peer_registering_handler(_: SocketAddr, _: &UdpSocket, _: &str) {
    ///     // do something
    /// }
    ///
    /// let event_handler = EventHandler::new();
    /// event_handler.set_pong_handler(peer_registering_handler);
    /// ```
    pub fn set_peer_registering_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// fn register_ack_handler(_: SocketAddr, _: &UdpSocket, _: &str) {
    ///     // do something
    /// }
    ///
    /// let event_handler = EventHandler::new();
    /// event_handler.set_register_ack_handler(register_ack_handler);
    /// ```
    pub fn set_register_ack_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
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
    /// use blockchain_network::event::EventHandler;
    /// use std::net::{UdpSocket, SocketAddr};
    ///
    /// fn register_handler(_: SocketAddr, _: &UdpSocket, _: &str) {
    ///     // do something
    /// }
    ///
    /// let event_handler = EventHandler::new();
    /// event_handler.set_register_handler(register_handler);
    /// ```
    pub fn set_register_handler(mut self, function: fn(SocketAddr, &UdpSocket, &str)) -> Self {
        self.register_handler = function;
        self
    }
}