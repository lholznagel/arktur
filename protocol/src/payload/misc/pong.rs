use payload::Payload;

/// Model for the event `Pong`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PongPayload;

impl Payload for PongPayload {
    fn new() -> Self {
        PongPayload
    }

    fn parse(_bytes: Vec<Vec<u8>>) -> Self {
        PongPayload
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![0]
    }
}