extern crate blockchain_network;

use blockchain_network::UdpClient;
use std::net::UdpSocket;
use std::str;
use std::thread;

fn main() {
    let network = UdpClient::new();
    let udp = network.start();
    println!("Running on port {:?}", udp.local_addr().unwrap().port());

    //let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
    //udp.send_to(&[0; 10], "0.0.0.0:50552").expect("couldn't send data");

    /*let mut buf = [0; 4096];
    let (_, src_addr) = udp.recv_from(&mut buf).expect("Didn't receive data");*/

    udp.send_to(&[0; 10], "gacrux.io:51000").expect("couldn't send data");

    loop {
        let mut buf = [0; 4096];
        match udp.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    println!("{}", str::from_utf8(&buf).unwrap_or(""));
                });
                udp.send_to(&[0; 10], src).expect("couldn't send data");
            }
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }

    /*println!(
        "Got message from {}. Message: {}",
        src_addr,
        str::from_utf8(&buf).unwrap_or("")
    );*/
}
