#[macro_use]
extern crate criterion;
extern crate carina_core_protocol;
extern crate sodiumoxide;

use carina_core_protocol::{MessageBuilder, Nacl, Payload};
use carina_core_protocol::payloads::EmptyPayload;
use criterion::Criterion;
use sodiumoxide::crypto::box_;

criterion_group!(benches, bench_payload_empty);
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