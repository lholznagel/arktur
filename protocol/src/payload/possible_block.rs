use payload::PayloadModel;

/// Model for the event `PossibleBlock`
#[derive(Debug, PartialEq)]
pub struct PossibleBlockPayload;

impl PayloadModel for PossibleBlockPayload {
    fn new() -> Self {
        PossibleBlockPayload
    }

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        PossibleBlockPayload
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}