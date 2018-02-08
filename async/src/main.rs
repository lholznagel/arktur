#![deny(deprecated)]
extern crate futures_cpupool;

use futures_cpupool::CpuPool;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct GlobalState {
    counter: u8,
    content: String
}

fn long_running_task() -> bool {
    println!("Start sleep");
    thread::sleep(Duration::from_secs(2));
    true
}

fn main() {
    let data = Arc::new(Mutex::new(GlobalState { counter: 0, content: String::from("") }));

    // set up everything for threading
    let pool = CpuPool::new_num_cpus();

    let mut threads = Vec::new();

    // setup udp
    let socket = UdpSocket::bind("127.0.0.1:13337").unwrap();
    
    loop {
        let mut buffer = [0; 1024];

        match socket.recv_from(&mut buffer) {
            Ok((bytes, _)) => {
                println!("{}", bytes);
                let counter = Arc::clone(&data);
                let prime = pool.spawn_fn((move || {
                    println!("Start");
                    long_running_task();

                    {
                        let mut state = counter.lock().unwrap();
                        state.counter += 1;
                        state.content = String::from("Hello, its me");
                    }

                    let res: Result<bool, ()> = Ok(true);
                    println!("Done");
                    res
                }));
                threads.push(prime);
                println!("{:?}", data)
            },
            Err(e) => println!("Error {:?}", e)
        };
    }
}
