use payload::PayloadModel;

/// Model for the event `Register`
#[derive(Debug, PartialEq)]
pub struct RegisterPayload;

impl PayloadModel for RegisterPayload {
    fn new() -> Self {
        RegisterPayload
    }

    fn parse(_bytes: Vec<&[u8]>) -> Self {
        RegisterPayload
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        vec![0]
    }
}