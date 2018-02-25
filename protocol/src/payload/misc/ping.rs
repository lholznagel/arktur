use payload::Payload;

/// Model for the event `Ping`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PingPayload;

impl Payload for PingPayload {
    fn new() -> Self {
        PingPayload
    }

    fn parse(_bytes: Vec<Vec<u8>>) -> Self {
        PingPayload
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![0]
    }
}