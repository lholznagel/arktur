use std::net::UdpSocket;

/// Trait containing all available hooks, clients can listen to
///
/// # Example how to implement. (Taken from `empty.rs`)
///
/// ```
/// use blockchain_hooks::Hooks;
/// use std::net::UdpSocket;
/// 
/// pub struct Empty;
/// 
/// impl Hooks for Empty {
///     fn on_ping(&self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_pong(&self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_register_hole_puncher(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_register_hole_puncher_ack(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_register_peer(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_register_peer_ack(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_data_for_block(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_new_block(&self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_possible_block(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_validate_hash(&self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_validated_hash(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_found_block(&self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
///
///     fn on_explore_network(&mut self, _: UdpSocket, _: Vec<u8>, _: String) {
///         // handle hook
///     }
/// }
/// ```
pub trait Hooks {
    /// Executed on a `PING` event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_ping(&self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `PONG` event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_pong(&self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `REGISTER_HOLE_PUNCHER` event
    /// Code: 16
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_hole_puncher(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `REGISTER_HOLE_PUNCHER_ACK` event
    /// Code: 17
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_hole_puncher_ack(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `REGISTER_PEER` event
    /// Code: 18
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_peer(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `REGISTER_PEER_ACK` event
    /// Code: 19
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register_peer_ack(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `DATA_FOR_BLOCK` event
    /// Code: 32
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_data_for_block(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `NEW_BLOCK` event
    /// Code: 33
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_new_block(&self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `POSSIBLE_BLOCK` event
    /// Code: 34
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_possible_block(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `VALIDATE_HASH` event
    /// Code: 35
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_validate_hash(&self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `VALIDATED_HASH` event
    /// Code: 36
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_validated_hash(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `FOUND_BLOCK` event
    /// Code: 37
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_found_block(&self, udp: UdpSocket, message: Vec<u8>, source: String);

    /// Executed on a `EXPLORE_NETWORK` event
    /// Code: 240
    ///
    /// # Parameters
    ///
    /// - `udp` - Open udp connection to send an answer
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_explore_network(&mut self, udp: UdpSocket, message: Vec<u8>, source: String);
}