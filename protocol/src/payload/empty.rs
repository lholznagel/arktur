use failure::Error;
use payload::Payload;

/// Empty payload
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmptyPayload;

impl Payload for EmptyPayload {
    fn new() -> Self {
        EmptyPayload
    }

    fn parse(_: Vec<Vec<u8>>) -> Result<Self, Error> {
        Ok(EmptyPayload)
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![0]
    }
}