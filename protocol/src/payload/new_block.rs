use payload::PayloadModel;

/// Model for the event `NewBlock`
#[derive(Debug, PartialEq)]
pub struct NewBlockPayload;

impl PayloadModel for NewBlockPayload {
    fn new() -> Self {
        NewBlockPayload
    }

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        NewBlockPayload
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}