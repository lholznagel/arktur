use std::mem::transmute;

/// Contains functions for parsing
pub struct Parser;

impl Parser {
    /// Splits the payload and saves it into a vector
    ///
    /// # Parameters
    ///
    /// - `payload: &[u8]` - raw payload
    ///
    /// # Returns
    ///
    /// Vec<Vec<u8>> vector of vector containing the parsed payload
    pub fn parse_payload(payload: &[u8]) -> Vec<Vec<u8>> {
        let mut index: u64 = 0;
        let mut complete = Vec::new();

        if !payload.is_empty() {
            loop {
                if index == payload.len() as u64 {
                    break;
                }

                let mut current = Vec::new();
                let current_length = payload[index as usize];

                for i in (index + 1)..(index + current_length as u64 + 1) {
                    current.push(payload[i as usize]);
                    index += 1;
                }

                index += 1;
                complete.push(current);
            }
        }

        complete
    }

    /// Converts an array of u8 values to a u16
    ///
    /// # Parameters
    ///
    /// - `value: &[u8]` - byte array
    ///
    /// # Returns
    ///
    /// Given u8 array as u16
    pub fn u8_to_u16(value: &[u8]) -> u16 {
        let mut value_byte: [u8; 2] = [0u8, 0u8];
        for i in 0..2 {
            value_byte[i] = value[i];
        }
        unsafe {
            transmute::<[u8; 2], u16>(value_byte)
        }
    }

    /// Converts an array of u8 values to a u32
    ///
    /// # Parameters
    ///
    /// - `value: &[u8]` - byte array
    ///
    /// # Returns
    ///
    /// Given u8 array as u32
    pub fn u8_to_u32(value: &[u8]) -> u32 {
        let mut value_byte: [u8; 4] = [0u8, 0u8, 0u8, 0u8];
        for i in 0..4 {
            value_byte[i] = value[i];
        }
        unsafe {
            transmute::<[u8; 4], u32>(value_byte)
        }
    }

    /// Converts an array of u8 values to a u64
    ///
    /// # Parameters
    ///
    /// - `value: &[u8]` - byte array
    ///
    /// # Returns
    ///
    /// Given u8 array as u64
    pub fn u8_to_u64(value: &[u8]) -> u64 {
        let mut value_byte: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
        for i in 0..8 {
            value_byte[i] = value[i];
        }
        unsafe {
            transmute::<[u8; 8], u64>(value_byte)
        }
    }

    /// Combines an string overflow back together
    ///
    /// # Parameters
    ///
    /// - `count: u8` - amount of splitted strings
    /// - `start: u8` - index of the first splitted string
    /// - `value: Vec<Vec<u8>>` - complete payload
    ///
    /// # Return
    ///
    /// Vector containing all splitted strings together
    pub fn string_overflow(count: u8, start: u8, value: Vec<Vec<u8>>) -> Vec<u8> {
        let mut content = Vec::new();
        for i in 0..count {
            content.extend(value[(start + i) as usize].iter());
        };
        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_to_u64() {
        let result = Parser::u8_to_u64(&[185u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(result, 1465);
    }

    #[test]
    fn test_u8_to_u64_too_long() {
        let result = Parser::u8_to_u64(&[185u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(result, 1465);
    }
}