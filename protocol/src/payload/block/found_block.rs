use payload::PayloadModel;

/// Model for the event `FoundBlock`
#[derive(Debug, PartialEq)]
pub struct FoundBlockPayload;

impl PayloadModel for FoundBlockPayload {
    fn new() -> Self {
        Self{}
    }

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        Self {}
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}