#[macro_use]
extern crate nom;

const CHARS: &'static [u8] = b"01234567890ABCDEF";

named!(parse_protocol<&[u8], (u8, u8, u16, u16, u16)>, bits!(tuple!(take_bits!(u8, 8), take_bits!(u8, 8), take_bits!(u16, 16), take_bits!(u16, 16), take_bits!(u16, 16))));

#[derive(Debug, PartialEq)]
pub struct BlockchainProtocol {
    event_code: u8,
    status_code: u8,
    id: u16,
    ttl: u16,
    data_length: u16
}

pub fn parse(to_parse: &[u8]) -> BlockchainProtocol {
    let parsed = parse_protocol(to_parse).to_result().unwrap();

    BlockchainProtocol {
        event_code: parsed.0,
        status_code: parsed.1,
        id: parsed.2,
        ttl: parsed.3,
        data_length: parsed.4
    }
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut v = Vec::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte &0xf) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_u8() {
        let expected = BlockchainProtocol {
            event_code: 1,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0];
        let result = parse(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_hex() {
        let expected = BlockchainProtocol {
            event_code: 1,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0
        };

        let data = &[0x01, 0x02, 0xFF, 0xFF, 0x05, 0x39, 0x00, 0x00];
        let result = parse(data);
        assert_eq!(result, expected);
    }
}