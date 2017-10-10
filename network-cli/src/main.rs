extern crate blockchain_network;

use blockchain_network::NetworkConnect;
use std::str;

fn main() {
    let network = NetworkConnect::new();
    let udp = network.start();
    println!("Running on port {:?}", udp.local_addr().unwrap().port());

    let mut buf = [0; 4096];
    let (_, src_addr) = udp.recv_from(&mut buf).expect("Didn't receive data");

    println!("Got message from {}. Message: {}", src_addr, str::from_utf8(&buf).unwrap_or(""));
}
