#[macro_use]
extern crate quickcheck;
extern crate blockchain_hooks;
extern crate blockchain_protocol;

use blockchain_hooks::EventCodes;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::*;

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
            .set_event_code(EventCodes::FoundBlock)
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
            .set_event_code(EventCodes::NewBlock)
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
            .set_event_code(EventCodes::PossibleBlock)
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
            .set_event_code(EventCodes::ValidateHash)
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
            .set_event_code(EventCodes::ValidatedHash)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<ValidatedHashPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(index, blockchain_parsed.payload.index);
        assert_eq!(hash, blockchain_parsed.payload.hash);
        true
    }
}

quickcheck! {
    fn test_peer_registering(addr: String) -> bool {
        let addr = addr;

        let payload = PeerRegisteringPayload {
            addr: addr.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<PeerRegisteringPayload>::new()
            .set_event_code(EventCodes::PeerRegistering)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<PeerRegisteringPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(addr, blockchain_parsed.payload.addr);
        true
    }
}

quickcheck! {
    fn test_register_ack(addr: String) -> bool {
        let addr = addr;

        let payload = RegisterAckPayload {
            addr: addr.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<RegisterAckPayload>::new()
            .set_event_code(EventCodes::AckRegister)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(addr, blockchain_parsed.payload.addr);
        true
    }
}

quickcheck! {
    fn test_register(name: String) -> bool {
        let name = name;

        let payload = RegisterPayload {
            name: name.clone()
        };

        let blockchain_protocol = BlockchainProtocol::<RegisterPayload>::new()
            .set_event_code(EventCodes::Register)
            .set_status_code(StatusCodes::Ok)
            .set_payload(payload)
            .build();

        let blockchain_parsed = BlockchainProtocol::<RegisterPayload>::from_bytes(&blockchain_protocol).unwrap();
        assert_eq!(name, blockchain_parsed.payload.name);
        true
    }
}