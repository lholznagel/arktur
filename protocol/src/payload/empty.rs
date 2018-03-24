use payload::Payload;
use errors::ParseErrors;

/// Empty payload
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmptyPayload;

impl Payload for EmptyPayload {
    fn new() -> Self {
        EmptyPayload
    }

    fn parse(_: Vec<Vec<u8>>) -> Result<Self, ParseErrors> {
        Ok(EmptyPayload)
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![0]
    }
}