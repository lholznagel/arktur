use payload::PayloadModel;

/// Model for the event `Ping`
#[derive(Clone, Debug, PartialEq)]
pub struct PingPayload;

impl PayloadModel for PingPayload {
    fn new() -> Self {
        PingPayload
    }

    fn parse(_bytes: Vec<Vec<u8>>) -> Self {
        PingPayload
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}