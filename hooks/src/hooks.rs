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
///     fn on_ping(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_pong(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_register(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_ack_register(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_peer_registering(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_new_block(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_possible_block(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_validate_hash(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_validated_hash(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
///
///     fn on_found_block(&self, _: Vec<u8>, _: String) -> Vec<u8> {
///         // handle hook and give a vector back
///         // the given vector is send back to the source address
///         // with an empty vector, no response is send to the source address
///         Vec::new() 
///     }
/// }
/// ```
pub trait Hooks {
    /// Executed on a `PING` event
    /// Code: 0
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_ping(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `PONG` event
    /// Code: 1
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_pong(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `Register` event
    /// Mostly server will listen to this
    /// Code: 16
    ///
    /// # Parameters
    ///
    /// - `udp` - Open UDP connection to send messages to other peers
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_register(&mut self, udp: &UdpSocket, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `ACK_REGISTER` event
    /// Code: 17
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_ack_register(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `PEER_REGISTERING` event
    /// Code: 18
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_peer_registering(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `NEW_BLOCK` event
    /// Code: 32
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_new_block(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `POSSIBLE_BLOCK` event
    /// Code: 33
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_possible_block(&mut self, udp: &UdpSocket, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `VALIDATE_HASH` event
    /// Code: 34
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_validate_hash(&self, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `VALIDATED_HASH` event
    /// Code: 35
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_validated_hash(&mut self, udp: &UdpSocket, message: Vec<u8>, source: String) -> Vec<u8>;

    /// Executed on a `FOUND_BLOCK` event
    /// Code: 36
    ///
    /// # Parameters
    ///
    /// - `message` - Raw message. Needs to be parsed, before usage
    /// - `source` - source address, that send this message
    fn on_found_block(&self, message: Vec<u8>, source: String) -> Vec<u8>;
}