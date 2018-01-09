use payload::PayloadModel;
use std::str;

/// Model for the event `FoundBlock`
#[derive(Debug, PartialEq)]
pub struct FoundBlockPayload {
    /// Index of the block
    pub index: u64,
    /// Content of the block
    pub content: String,
    /// Timestamp the block was created
    pub timestamp: i64,
    /// Nonce for this block
    pub nonce: u64,
    /// Hash of the previous block
    pub prev: String,
    /// Hash of this block
    pub hash: String
}

impl PayloadModel for FoundBlockPayload {
    fn new() -> Self {
        Self {
            index: 0,
            content: String::from(""),
            timestamp: 0,
            nonce: 0,
            prev: String::from(""),
            hash: String::from("")
        }
    }

    fn parse(bytes: Vec<Vec<u8>>) -> Self {
        Self {
            index: String::from(str::from_utf8(&bytes[0]).unwrap()).parse::<u64>().unwrap(),
            content: String::from(str::from_utf8(&bytes[1]).unwrap()),
            timestamp: String::from(str::from_utf8(&bytes[2]).unwrap()).parse::<i64>().unwrap(),
            nonce: String::from(str::from_utf8(&bytes[3]).unwrap()).parse::<u64>().unwrap(),
            prev: String::from(str::from_utf8(&bytes[4]).unwrap()),
            hash: String::from(str::from_utf8(&bytes[5]).unwrap()),
        }
    }

    fn length(&self) -> u16 {
        0
    }

    fn as_bytes(self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        result.push(self.index.to_string().into_bytes().len() as u8);
        for i in self.index.to_string().into_bytes() {
            result.push(i);
        }

        result.push(self.content.clone().into_bytes().len() as u8);
        for i in self.content.clone().into_bytes() {
            result.push(i);
        }

        result.push(self.timestamp.to_string().into_bytes().len() as u8);
        for i in self.timestamp.to_string().into_bytes() {
            result.push(i);
        }

        result.push(self.nonce.to_string().into_bytes().len() as u8);
        for i in self.nonce.to_string().into_bytes() {
            result.push(i);
        }

        result.push(self.prev.clone().into_bytes().len() as u8);
        for i in self.prev.clone().into_bytes() {
            result.push(i);
        }

        result.push(self.hash.clone().into_bytes().len() as u8);
        for i in self.hash.clone().into_bytes() {
            result.push(i);
        }
        result.push(0);
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BlockchainProtocol;

    fn parse_payload(payload: Vec<u8>) -> Vec<Vec<u8>> {
        let mut index = 0;
        let mut complete = Vec::new();

        loop {
            let mut current = Vec::new();
            let current_length = payload[(index) as usize];

            if payload[(index) as usize] == 0 {
                break;
            }

            for i in (index + 1)..(index + current_length + 1) {
                current.push(payload[i as usize]);
                index += 1;
            }

            index += 1;
            complete.push(current);
        }

        complete
    }

    #[test]
    fn test_building_and_parsing() {
        let index = 1465;
        let content = String::from("Some string");
        let timestamp = 5825525;
        let nonce = 41684984;
        let prev = String::from("sdghnregneiurngnwg48g4g4erg46e4hh");
        let hash = String::from("asdmhgoirmhoiremh54651greher4h545");

        let mut found_block = FoundBlockPayload::new();
        found_block.index = index.clone();
        found_block.content = content.clone();
        found_block.timestamp = timestamp.clone();
        found_block.nonce = nonce.clone();
        found_block.prev = prev.clone();
        found_block.hash = hash.clone();
        let found_block = found_block.as_bytes();

        let complete = parse_payload(found_block);

        let parsed = FoundBlockPayload::parse(complete);

        assert_eq!(index, parsed.index);
        assert_eq!(content, parsed.content);
        assert_eq!(timestamp, parsed.timestamp);
        assert_eq!(nonce, parsed.nonce);
        assert_eq!(prev, parsed.prev);
        assert_eq!(hash, parsed.hash);
    }

    /*quickcheck! {
        fn test_quickcheck(index: u64, content: String, timestamp: i64, nonce: u64, prev: String, hash: String) -> bool {
            let index = index;
            let content = content;
            let timestamp = timestamp;
            let nonce = nonce;
            let prev = prev;
            let hash = hash;

            let mut found_block = FoundBlockPayload::new();
            found_block.index = index.clone();
            found_block.content = content.clone();
            found_block.timestamp = timestamp.clone();
            found_block.nonce = nonce.clone();
            found_block.prev = prev.clone();
            found_block.hash = hash.clone();
            let found_block = found_block.as_bytes();

            println!("{:?}", index);

            println!("{:?}", found_block);

            let parser = parse_delimited(&found_block).to_result().unwrap();
            let parsed = FoundBlockPayload::parse(parser);

            assert_eq!(index, parsed.index);
            assert_eq!(content, parsed.content);
            assert_eq!(timestamp, parsed.timestamp);
            assert_eq!(nonce, parsed.nonce);
            assert_eq!(prev, parsed.prev);
            assert_eq!(hash, parsed.hash);

            true
        }
    }*/
}