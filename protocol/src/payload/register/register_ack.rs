use payload::PayloadModel;
use std::str;

/// Model for the event `RegisterAck`
#[derive(Clone, Debug, PartialEq)]
pub struct RegisterAckPayload {
    /// Address of another peer
    pub addr: String,
}

impl RegisterAckPayload {
    /// Sets the address another peer
    ///
    /// # Parameter
    ///
    /// - `addr` - Address of another peer
    ///
    /// # Return
    ///
    /// Updated instance
    pub fn set_addr(mut self, addr: String) -> Self {
        self.addr = addr;
        self
    }
}

impl PayloadModel for RegisterAckPayload {
    fn new() -> Self {
        RegisterAckPayload { addr: String::from("") }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        if bytes.len() > 0 {
            RegisterAckPayload { addr: String::from(str::from_utf8(&bytes[0]).unwrap()) }
        } else {
            RegisterAckPayload { addr: String::from("") }
        }
    }

    fn length(&self) -> u16 {
        let mut result = 0;
        if self.addr.len() != 0 {
            result = self.addr.len().to_string().parse::<u16>().unwrap() + 2
        }

        result
    }

    fn as_bytes(self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        if self.addr.to_string() != "" {
            // 126 as char equals ~
            result.push(126);

            for i in self.addr.to_string().into_bytes() {
                result.push(i);
            }

            result.push(126);
        }

        result
    }
}