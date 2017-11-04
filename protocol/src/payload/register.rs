use payload::PayloadParser;
use std::str;

#[derive(Debug, PartialEq)]
pub struct RegisterPayload {
    addr: String,
}

impl RegisterPayload {
    pub fn set_addr(mut self, addr: String) -> Self {
        self.addr = addr;
        self
    }
}

impl PayloadParser for RegisterPayload {
    fn new() -> Self {
        RegisterPayload { addr: String::from("") }
    }

    fn parse(bytes: Vec<&[u8]>) -> Self {
        if bytes.len() > 0 {
            RegisterPayload { addr: String::from(str::from_utf8(bytes[0]).unwrap()) }
        } else {
            RegisterPayload { addr: String::from("") }
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