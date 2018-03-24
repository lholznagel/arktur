//! Useful functions for parsing an array of bytes
use protocol::ParseErrors;

use std::mem::transmute;
use std::str;

/// Splits the payload and saves it into a vector
///
/// # Parameters
///
/// - `payload: &[u8]` - raw payload
///
/// # Example
///
/// The byte array
/// `&[3, 65, 66, 67, 5, 68, 69, 70, 71, 72]`
///
/// will be converted to
/// `[[65, 66, 67], [68, 69, 70, 71, 72]]`
///
/// The numbers 3 and 5 determine how many items are grouped
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
/// This function is unsafe!
///
/// # Parameters
///
/// - `value: &[u8]` - byte array
///
/// # Returns
///
/// Given u8 array as u16
pub fn u8_to_u16(value: &[u8]) -> Result<u16, ParseErrors> {
    if value.len() != 2 {
        return Err(ParseErrors::NotEnoughBytes);
    }

    unsafe {
        Ok(transmute::<[u8; 2], u16>([value[0], value[1]]))
    }
}

/// Converts an array of u8 values to a u32
///
/// This function is unsafe!
/// 
/// # Parameters
///
/// - `value: &[u8]` - byte array
///
/// # Returns
///
/// Given u8 array as u32
pub fn u8_to_u32(value: &[u8]) -> Result<u32, ParseErrors> {
    if value.len() != 4 {
        return Err(ParseErrors::NotEnoughBytes);
    }

    unsafe {
        Ok(transmute::<[u8; 4], u32>([value[0], value[1], value[2], value[3]]))
    }
}

/// Converts an array of u8 values to a u64
///
/// This function is unsafe!
///
/// # Parameters
///
/// - `value: &[u8]` - byte array
///
/// # Returns
///
/// Given u8 array as u64
pub fn u8_to_u64(value: &[u8]) -> Result<u64, ParseErrors> {
    if value.len() != 8 {
        return Err(ParseErrors::NotEnoughBytes);
    }

    unsafe {
        Ok(transmute::<[u8; 8], u64>([value[0], value[1], value[2], value[3], value[4], value[5], value[6], value[7]]))
    }
}

/// Converts an array of u8 values to a string
///
/// # Parameters
///
/// - `value: &[u8]` - byte array
///
/// # Returns
///
/// Given u8 array as string
pub fn u8_to_string(value: &[u8]) -> String {
    str::from_utf8(value)
        .unwrap_or("")
        .to_string()
}

/// Combines an string overflow back together
///
/// # Parameters
///
/// - `value: Vec<Vec<u8>>` - complete payload
///
/// # Return
///
/// Vector containing all splitted strings together
pub fn string_overflow(values: &[Vec<u8>]) -> Vec<u8> {
    let mut content = Vec::new();
    for current in values {
        content.extend(current.iter());
    };
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_payload_single() {
        // [3, 65, 66, 67] -> [3, "A", "B", "C"]
        let result = parse_payload(&[3, 65, 66, 67]);
        assert_eq!(result, [[65, 66, 67]])
    }

    #[test]
    fn test_parse_payload_multi() {
        // [3, 65, 66, 67, 3, 68, 69, 70] -> [3, "A", "B", "C", 3, "D", "E", "F"]
        let result = parse_payload(&[3, 65, 66, 67, 3, 68, 69, 70]);
        assert_eq!(result, [[65, 66, 67], [68, 69, 70]])
    }

    #[test]
    fn test_u8_to_u16_success() {
        let result = u8_to_u16(&[185u8, 5u8]);
        assert_eq!(result.unwrap(), 1465);
    }

    #[test]
    fn test_u8_to_u16_fail_low() {
        match u8_to_u16(&[185u8]) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_u8_to_u16_fail_high() {
        match u8_to_u16(&[185u8, 5u8, 10u8]) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_u8_to_u32() {
        let result = u8_to_u32(&[185u8, 5u8, 0u8, 0u8]);
        assert_eq!(result.unwrap(), 1465);
    }

    #[test]
    fn test_u8_to_u32_fail_low() {
        match u8_to_u32(&[185u8, 5u8]) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_u8_to_u32_fail_high() {
        match u8_to_u32(&[185u8, 5u8, 10u8, 0u8, 232u8]) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_u8_to_u64() {
        let result = u8_to_u64(&[185u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        assert_eq!(result.unwrap(), 1465);
    }

    #[test]
    fn test_u8_to_u64_fail_low() {
        match u8_to_u64(&[185u8, 5u8, 0u8, 0u8]) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_u8_to_u64_fail_high() {
        match u8_to_u64(&[185u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]) {
            Ok(_)  => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_string_overflow_empty() {
        let result = string_overflow(&[Vec::new()]);
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_string_overflow_single() {
        let result = string_overflow(&[vec![65, 66, 67]]);
        assert_eq!(result, vec![65, 66, 67]);
    }

    #[test]
    fn test_string_overflow_multi() {
        let result = string_overflow(&[vec![65, 66, 67], vec![68, 69, 70], vec![71, 72, 73]]);
        assert_eq!(result, vec![65, 66, 67, 68, 69, 70, 71, 72, 73]);
    }
}