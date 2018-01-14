
/// Trait that is needed by every model that represents
/// a payload of an event
pub trait Payload: Clone {
    /// Creates a new empty instance of the model
    fn new() -> Self;

    /// Should parse the given vector of bytes
    /// to the payload model
    ///
    /// # Parameters
    ///
    /// - `bytes: Vec<Vec<u8>>` - parsed payload
    ///
    /// # Returns
    ///
    /// Instance of the payload model
    fn parse(bytes: Vec<Vec<u8>>) -> Self;

    /// Should convert the current payload model to a
    /// vector of bytes
    ///
    /// # Returns
    /// 
    /// Vector of bytes that represent the payload model
    fn to_bytes(self) -> Vec<u8>;
}
