/// Converts an vector of string to a vector of u8
///
/// Used for parsing the payloads to bytes
#[derive(Debug)]
pub struct PayloadBuilder {
    byte_vec: Vec<u8>
}

impl PayloadBuilder {
    /// Creates a new instance of the PayloadBuilder
    pub fn new() -> Self {
        Self {
            byte_vec: Vec::new()
        }
    }

    /// Adds a new string
    ///
    /// # Parameters
    ///
    /// - `values: String` - value to add
    ///
    /// # Return
    ///
    /// Updated instance of PayloadBuilder
    pub fn add_string(mut self, value: String) -> Self {
        let value_byte = value.clone().into_bytes();

        self.byte_vec.push(value_byte.len() as u8);
        self.byte_vec.extend(value_byte.iter());
        self
    }

    /// Adds a new u8 value
    ///
    /// # Parameters
    ///
    /// - `value: u8` - value to add
    ///
    /// # Return
    ///
    /// Updated instance of PayloadBuilder
    pub fn add_u8(mut self, value: u8) -> Self {
        self.byte_vec.push(1);
        self.byte_vec.push(value);
        self
    }

    /// Adds a new u64 value
    ///
    /// # Parameters
    ///
    /// - `value: u64` - value to add
    ///
    /// # Return
    ///
    /// Updated instance of PayloadBuilder
    pub fn add_u64(mut self, value: u64) -> Self {
        let b1 = ((value >> 56) & 0xFF) as u8;
        let b2 = ((value >> 48) & 0xFF) as u8;
        let b3 = ((value >> 40) & 0xFF) as u8;
        let b4 = ((value >> 32) & 0xFF) as u8;
        let b5 = ((value >> 24) & 0xFF) as u8;
        let b6 = ((value >> 16) & 0xFF) as u8;
        let b7 = ((value >> 8) & 0xFF) as u8;
        let b8 = (value & 0xFF) as u8;

        self.byte_vec.push(8);
        self.byte_vec.extend(vec![b8, b7, b6, b5, b4, b3, b2, b1].iter());
        self
    }

    /// Adds a new string that can be longer than 255 chars
    ///
    /// # Parameters
    ///
    /// - `value: String` - value to add
    ///
    /// # Return
    ///
    /// Updated instance of PayloadBuilder
    pub fn add_string_overflow(mut self, value: String) -> Self {
        let value_byte = value.clone().into_bytes();
        let mut current_index = 0;

        loop {
            if value_byte.len() - current_index == 0 {
                break;
            }

            if value_byte.len() - current_index > 255 {
                self.byte_vec.push(255);

                for _ in current_index..(current_index + 255) {
                    self.byte_vec.push(value_byte[current_index]);
                    current_index += 1;
                }
            } else {
                let remaining = value_byte.len() - current_index;
                self.byte_vec.push(remaining as u8);

                for _ in current_index..(current_index + remaining) {
                    self.byte_vec.push(value_byte[current_index]);
                    current_index += 1;
                }
            }
        };
        self
    }

    /// Converts the string vector to byte vector
    ///
    /// # Returns
    ///
    /// All added strings as byte vector
    pub fn build(mut self) -> Vec<u8> {
        self.byte_vec.push(0);
        self.byte_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_vec_to_byte_vec() {
        let result = PayloadBuilder::new()
            .add_string(String::from("SomeString"))
            .add_string(String::from("SomeOtherString"))
            .build();

        let expected = vec![10, 83, 111, 109, 101, 83, 116, 114, 105, 110, 103, 15, 83, 111, 109, 101, 79, 116, 104, 101, 114, 83, 116, 114, 105, 110, 103, 0];
        assert_eq!(expected, result);
    }
}