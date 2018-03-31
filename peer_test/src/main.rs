#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! Ctate for peer integration tests

extern crate carina_hooks;
extern crate carina_peer;
extern crate carina_protocol;
extern crate futures;
extern crate futures_cpupool;

mod connection;
mod test_case;

use test_case::TestCase;

use futures::future::Future;
use futures_cpupool::CpuPool;

fn main() {
    let pool = CpuPool::new_num_cpus();
    let mut thread_storage = Vec::new();

    thread_storage.push(connection::RegisterAck::execute(&pool));

    for thread in thread_storage {
        if thread.wait().unwrap() {
            println!("ok");
        } else {
            println!("fail");
        }
    }
}