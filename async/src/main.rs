
#![deny(deprecated)]
extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

use std::time::Duration;
use std::thread;

fn is_prime() -> bool {
    println!("Start sleep");
    thread::sleep(Duration::from_secs(60));
    true
}

fn is_prime_short() -> bool {
    println!("Start sleep");
    thread::sleep(Duration::from_secs(10));
    true
}

fn main() {
    // set up a thread pool
    let pool = CpuPool::new_num_cpus();

    // spawn our computation, getting back a *future* of the answer
    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime();

        // For reasons we'll see later, we need to return a Result here
        let res: Result<bool, ()> = Ok(prime);
        res
    });

    println!("Created the future");

    let mut primes = vec![];
    for i in 0..10 {
        println!("{}", i);
        let prime = pool.spawn_fn((|| {
            is_prime_short();
            let res: Result<bool, ()> = Ok(true);
            println!("Done");
            res
        }));
        primes.push(prime);
    }

    //let prime_future = futures::future::ok::<bool, ()>(true);
    if prime_future.wait().unwrap() {
        println!("Prime");
    } else {
        println!("Not prime");
    }
}
