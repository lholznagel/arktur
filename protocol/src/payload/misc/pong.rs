use payload::PayloadModel;

/// Model for the event `Pong`
#[derive(Clone, Debug, PartialEq)]
pub struct PongPayload;

impl PayloadModel for PongPayload {
    fn new() -> Self {
        PongPayload
    }

    fn parse(_bytes: Vec<Vec<u8>>) -> Self {
        PongPayload
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}