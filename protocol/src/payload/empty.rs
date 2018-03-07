use payload::Payload;

/// Model for the event `Ping`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmptyPayload;

impl Payload for EmptyPayload {
    fn new() -> Self {
        EmptyPayload
    }

    fn parse(_: Vec<Vec<u8>>) -> Self {
        EmptyPayload
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![0]
    }
}