#![feature(test)]

extern crate carina_protocol;
extern crate test;

use carina_protocol::Protocol;
use carina_protocol::nacl::Nacl;
use carina_protocol::payload::*;
use carina_protocol::payload::blocks::*;

use test::Bencher;

#[bench]
fn bench_empty(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let blockchain_protocol = Protocol::<EmptyPayload>::new()
            .set_event_code(0)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<EmptyPayload>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_punch(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = Punsh {
            address: String::from("SomeAddress")
        };

        let blockchain_protocol = Protocol::<Punsh>::new()
            .set_event_code(2)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<Punsh>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_get_block(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = GetBlock {
            block: String::from("SomeBlock")
        };

        let blockchain_protocol = Protocol::<GetBlock>::new()
            .set_event_code(130)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<GetBlock>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_get_block_ack(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = GetBlockAck {
            filename: String::from("SomeFilename"),
            index: 345678,
            timestamp: 2345790,
            nonce: 1234567890,
            prev: String::from("PrevHash"),
            hash: String::from("CurrentHash"),
            content: String::from("MySuperAwesomeContent")
        };

        let blockchain_protocol = Protocol::<GetBlockAck>::new()
            .set_event_code(131)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<GetBlockAck>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_block_data(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = BlockData {
            unique_key: String::from("asdafhgr"),
            content: String::from("asdasdasfagewg")
        };

        let blockchain_protocol = Protocol::<BlockData>::new()
            .set_event_code(37)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<BlockData>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_block_gen(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = BlockGen {
            index: 458648,
            timestamp: 321,
            sign_key: String::from("0000"),
            prev: String::from("agg43g34g"),
            content: String::from("gg4g43g43gg")
        };

        let blockchain_protocol = Protocol::<BlockGen>::new()
            .set_event_code(33)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<BlockGen>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_block_found(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = BlockFound {
            index: 6565,
            timestamp: 4516514521,
            nonce: 645246452,
            prev: String::from("asdasdasdasd"),
            hash: String::from("afht5ejh5hgg"),
            content: String::from("asdasdasfagewg")
        };

        let blockchain_protocol = Protocol::<BlockFound>::new()
            .set_event_code(37)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<BlockFound>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_hash_val(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = HashVal {
            index: 6456948,
            nonce: 64645,
            timestamp: 645645,
            prev: String::from("afdg3gergergerg"),
            content: String::from("wg3hhrthrhtrh")
        };

        let blockchain_protocol = Protocol::<HashVal>::new()
            .set_event_code(35)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<HashVal>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}

#[bench]
fn bench_hash_val_ack(b: &mut Bencher) {
    let mut our_nacl = Nacl::new();
    let there_nacl = Nacl::new();

    b.iter(|| {
        let payload = HashValAck {
            index: 245458,
            hash: String::from("safergrethgergregerg")
        };

        let blockchain_protocol = Protocol::<HashValAck>::new()
            .set_event_code(36)
            .set_payload(payload)
            .build(&mut our_nacl, &there_nacl.get_public_key());

        Protocol::<HashValAck>::from_bytes(&blockchain_protocol, &there_nacl, &our_nacl.get_public_key()).unwrap();
    });
}