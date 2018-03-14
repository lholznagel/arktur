#![feature(test)]

extern crate blockchain_protocol;
extern crate test;

use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::*;
use blockchain_protocol::payload::blocks::*;
use blockchain_protocol::payload::peers::*;

use test::Bencher;

#[bench]
fn bench_data_for_block(b: &mut Bencher) {
    b.iter(|| {
        let payload = BlockData {
            unique_key: String::from("asdafhgr"),
            content: String::from("asdasdasfagewg")
        };

        let blockchain_protocol = BlockchainProtocol::<BlockData>::new()
            .set_event_code(37)
            .set_payload(payload)
            .build();

        BlockchainProtocol::<BlockData>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_found_block(b: &mut Bencher) {
    b.iter(|| {
        let payload = BlockFound {
            index: 6565,
            timestamp: 4516514521,
            nonce: 645246452,
            prev: String::from("asdasdasdasd"),
            hash: String::from("afht5ejh5hgg"),
            content: String::from("asdasdasfagewg")
        };

        let blockchain_protocol = BlockchainProtocol::<BlockFound>::new()
            .set_event_code(37)
            .set_payload(payload)
            .build();

        BlockchainProtocol::<BlockFound>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_new_block(b: &mut Bencher) {
    b.iter(|| {
        let payload = BlockGen {
            index: 458648,
            timestamp: 321,
            sign_key: String::from("0000"),
            prev: String::from("agg43g34g"),
            content: String::from("gg4g43g43gg")
        };

        let blockchain_protocol = BlockchainProtocol::<BlockGen>::new()
            .set_event_code(33)
            .set_payload(payload)
            .build();

        BlockchainProtocol::<BlockGen>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_validate_hash(b: &mut Bencher) {
    b.iter(|| {
        let payload = HashVal {
            index: 6456948,
            nonce: 64645,
            timestamp: 645645,
            prev: String::from("afdg3gergergerg"),
            content: String::from("wg3hhrthrhtrh")
        };

        let blockchain_protocol = BlockchainProtocol::<HashVal>::new()
            .set_event_code(35)
            .set_payload(payload)
            .build();

        BlockchainProtocol::<HashVal>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_validated_hash(b: &mut Bencher) {
    b.iter(|| {
        let payload = HashValAck {
            index: 245458,
            hash: String::from("safergrethgergregerg")
        };

        let blockchain_protocol = BlockchainProtocol::<HashValAck>::new()
            .set_event_code(36)
            .set_payload(payload)
            .build();

        BlockchainProtocol::<HashValAck>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_ping(b: &mut Bencher) {
    b.iter(|| {
        let blockchain_protocol = BlockchainProtocol::<EmptyPayload>::new()
            .set_event_code(0)
            .build();

        BlockchainProtocol::<EmptyPayload>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_register_ack(b: &mut Bencher) {
    b.iter(|| {
        let payload = RegisterAckPayload {
            addresses: vec![String::from("geggwegwegwegweg")]
        };

        let blockchain_protocol = BlockchainProtocol::<RegisterAckPayload>::new()
            .set_event_code(17)
            .set_payload(payload)
            .build();

        BlockchainProtocol::<RegisterAckPayload>::from_bytes(&blockchain_protocol).unwrap();
    });
}

#[bench]
fn bench_register(b: &mut Bencher) {
    b.iter(|| {
        let blockchain_protocol = BlockchainProtocol::<EmptyPayload>::new()
            .set_event_code(16)
            .set_payload(EmptyPayload::new())
            .build();

        BlockchainProtocol::<EmptyPayload>::from_bytes(&blockchain_protocol).unwrap();
    });
}