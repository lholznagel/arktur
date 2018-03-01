#[macro_use]
extern crate quickcheck;
extern crate blockchain_protocol;

use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::*;

quickcheck! {
    fn test_data_for_block(unique_key: String, content: String) -> bool {
        let content = content;

        let payload = DataForBlockPayload {
            unique_key: unique_key.clone(),
            content: content.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<DataForBlockPayload>::new()
            .set_event_code(37)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<DataForBlockPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(unique_key, blockchain_parsed.payload.unique_key);
        assert_eq!(content, blockchain_parsed.payload.content);
        true
    }
}

quickcheck! {
    fn test_found_block(index: u64, timestamp: i64, nonce: u64, hash: String, prev: String, content: String) -> bool {
        let index = index;
        let timestamp = timestamp;
        let nonce = nonce;
        let prev = prev;
        let hash = hash;
        let content = content;

        let payload = FoundBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            hash: hash.clone(),
            content: content.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<FoundBlockPayload>::new()
            .set_event_code(37)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<FoundBlockPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(nonce, blockchain_parsed.payload.nonce);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        assert_eq!(content, blockchain_parsed.payload.content);
        true
    }
}

quickcheck! {
    fn test_new_block(index: u64, timestamp: i64, prev: String, sign_key: String, content: String) -> bool {
        let index = index;
        let timestamp = timestamp;
        let sign_key = sign_key;
        let prev = prev;
        let content = content;

        let payload = NewBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            sign_key: sign_key.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<NewBlockPayload>::new()
            .set_event_code(33)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<NewBlockPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(content, blockchain_parsed.payload.content);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(sign_key, blockchain_parsed.payload.sign_key);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        true
    }
}

quickcheck! {
    fn test_possible_block(index: u64, timestamp: i64, nonce: u64, hash: String, prev: String, content: String) -> bool {
        let index = index;
        let timestamp = timestamp;
        let nonce = nonce;
        let prev = prev;
        let hash = hash;
        let content = content;

        let payload = PossibleBlockPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            hash: hash.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<PossibleBlockPayload>::new()
            .set_event_code(34)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<PossibleBlockPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(nonce, blockchain_parsed.payload.nonce);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        assert_eq!(content, blockchain_parsed.payload.content);
        true
    }
}

quickcheck! {
    fn test_validate_hash(index: u64, timestamp: i64, nonce: u64, prev: String, content: String) -> bool {
        let index = index;
        let nonce = nonce;
        let timestamp = timestamp;
        let prev = prev;
        let content = content;

        let payload = ValidateHashPayload {
            index: index.clone(),
            timestamp: timestamp.clone(),
            nonce: nonce.clone(),
            prev: prev.clone(),
            content: content.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<ValidateHashPayload>::new()
            .set_event_code(35)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<ValidateHashPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(content, blockchain_parsed.payload.content);
        assert_eq!(timestamp, blockchain_parsed.payload.timestamp);
        assert_eq!(nonce, blockchain_parsed.payload.nonce);
        assert_eq!(prev, blockchain_parsed.payload.prev);
        true
    }
}

quickcheck! {
    fn test_validated_hash(index: u64, hash: String) -> bool {
        let index = index;
        let hash = hash;

        let payload = ValidatedHashPayload {
            index: index.clone(),
            hash: hash.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<ValidatedHashPayload>::new()
            .set_event_code(36)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<ValidatedHashPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        true
    }
}