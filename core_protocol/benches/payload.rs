#[macro_use]
extern crate criterion;
extern crate carina_core_protocol;
extern crate sodiumoxide;

use carina_core_protocol::{MessageBuilder, Nacl, Payload};
use carina_core_protocol::payloads::EmptyPayload;
use carina_core_protocol::payloads::block::*;
use criterion::Criterion;
use sodiumoxide::crypto::box_;

criterion_group!(benches, bench_payload_empty, bench_block_data, bench_generate_block);
criterion_main!(benches);

fn bench_payload_empty(c: &mut Criterion) {
    c.bench_function("bench_payload_empty", |b| b.iter(|| {
        let (_, oursk) = box_::gen_keypair();
        let (theirpk, _) = box_::gen_keypair();
        let mut nacl = Nacl::new(oursk);

        MessageBuilder::new()
            .set_event_code(1)
            .set_payload(EmptyPayload::new())
            .build(&mut nacl, &theirpk);
    }));
}

fn bench_block_data(c: &mut Criterion) {
    c.bench_function("bench_block_data", |b| b.iter(|| {
        let (_, oursk) = box_::gen_keypair();
        let (theirpk, _) = box_::gen_keypair();
        let mut nacl = Nacl::new(oursk);

        let mut payload = NewBlockContent::new();
        payload.content = String::from("SomeCoolContent");

        MessageBuilder::new()
            .set_event_code(64)
            .set_payload(payload)
            .build(&mut nacl, &theirpk);
    }));
}

fn bench_generate_block(c: &mut Criterion) {
    c.bench_function("bench_generate_block", |b| b.iter(|| {
        let (_, oursk) = box_::gen_keypair();
        let (theirpk, _) = box_::gen_keypair();
        let mut nacl = Nacl::new(oursk);

        let payload = GenerateBlock::block(0, "0".repeat(64), "a".repeat(100));
        MessageBuilder::new()
            .set_event_code(64)
            .set_payload(payload)
            .build(&mut nacl, &theirpk);
    }));
}