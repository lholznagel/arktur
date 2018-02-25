use payload::{Payload, PayloadBuilder};

/// Model for the event `Register`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegisterPayload;

impl Payload for RegisterPayload {
    fn new() -> Self {
        Self{}
    }

    fn parse(_: Vec<Vec<u8>>) -> Self {
        Self::new()
    }

    fn to_bytes(self) -> Vec<u8> {
        PayloadBuilder::new().build()
    }
}