use payload::PayloadParser;

#[derive(Debug, PartialEq)]
pub struct PingPayload;

impl PayloadParser for PingPayload {
    fn new() -> Self {
        PingPayload
    }

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        PingPayload
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}