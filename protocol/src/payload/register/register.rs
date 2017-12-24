use payload::PayloadModel;
use std::str;

/// Model for the event `Register`
#[derive(Debug, PartialEq)]
pub struct RegisterPayload {
    name: String,
}

impl RegisterPayload {
    /// Sets the name of the peer
    ///
    /// # Parameters
    ///
    /// - `name` - name of the peer
    ///
    /// # Return
    ///
    /// Updated instance of itself
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Gets the peer name
    ///
    /// # Return
    ///
    /// Name of the peer
    pub fn name(self) -> String {
        self.name
    }
}

impl PayloadModel for RegisterPayload {
    fn new() -> Self {
        RegisterPayload { name: String::from("") }
    }

    fn parse(bytes: Vec<&[u8]>) -> Self {
        if bytes.len() > 0 {
            RegisterPayload { name: String::from(str::from_utf8(bytes[0]).unwrap()) }
        } else {
            RegisterPayload { name: String::from("") }
        }
    }

    fn length(&self) -> u16 {
        let mut result = 0;
        if self.name.len() != 0 {
            result = self.name.len().to_string().parse::<u16>().unwrap() + 2
        }

        result
    }

    fn as_bytes(self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();

        if self.name.to_string() != "" {
            // 126 as char equals ~
            result.push(126);

            for i in self.name.to_string().into_bytes() {
                result.push(i);
            }

            result.push(126);
        }

        result
    }
}