#[macro_use]
extern crate nom;

pub mod hex;

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

#[cfg(test)]
mod tests {
    use super::*;
    use hex::*;

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

    #[test]
    fn test_simple_string_u8() {
        let expected = BlockchainProtocol {
            event_code: 1,
            status_code: 2,
            id: 65535,
            ttl: 1337,
            data_length: 0
        };

        let data = &[1, 2, 255, 255, 5, 57, 0, 0];
        let data = to_hex(data);
        println!("{:?}", data);
        let data = from_hex(data.as_str());
        let result = parse(data.as_slice());
        assert_eq!(result, expected);
    }
}