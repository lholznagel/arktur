/// Converts an vector of string to a vector of u8
///
/// Used for parsing the payloads to bytes
pub struct ByteBuilder {
    strings: Vec<String>
}

impl ByteBuilder {
    /// Creates a new instance of the ByteBuilder
    pub fn new() -> Self {
        Self {
            strings: Vec::new()
        }
    }

    /// Adds a new string
    ///
    /// # Parameters
    ///
    /// - `data: String` - data to parse as string
    ///
    /// # Return
    ///
    /// Updated instance of ByteBuilder
    pub fn add(mut self, data: String) -> Self {
        self.strings.push(data);
        self
    }

    /// Converts the string vector to byte vector
    ///
    /// # Returns
    ///
    /// All added strings as byte vector
    pub fn build(self) -> Vec<u8> {
        let mut result = Vec::new();

        for current in self.strings {
            let current_byte = current.into_bytes();

            result.push(current_byte.len() as u8);
            result.extend(current_byte.iter());
        }

        result.push(0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_vec_to_byte_vec() {
        let byte_builder = ByteBuilder::new();

        let result = byte_builder
            .add(String::from("SomeString"))
            .add(String::from("SomeOtherString"))
            .build();

        let expected = vec![10, 83, 111, 109, 101, 83, 116, 114, 105, 110, 103, 15, 83, 111, 109, 101, 79, 116, 104, 101, 114, 83, 116, 114, 105, 110, 103, 0];
        assert_eq!(expected, result);
    }
}