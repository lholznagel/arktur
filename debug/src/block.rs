use blockchain_hooks::{as_number, EventCodes};
use blockchain_protocol::Protocol;
use blockchain_protocol::payload::Payload;
use blockchain_protocol::payload::blocks::BlockData;

use clap::ArgMatches;
use std::net::UdpSocket;
use rand::random;

pub fn execute(args: &ArgMatches) {
    let mut peer_address = String::from("");
    if args.is_present("PEER_PORT") {
        peer_address.push_str(args.value_of("PEER_IP").unwrap());
        peer_address.push_str(":");
        peer_address.push_str(args.value_of("PEER_PORT").unwrap());
    }

    let mut payload = BlockData::new();
    payload.unique_key = (0..8).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect();
    
    if args.is_present("MESSAGE") {
        payload.content = args.value_of("MESSAGE").unwrap().to_string();
    } else {
        payload.content = "Super awesome message".to_string();
    }

    let request = Protocol::<BlockData>::new()
            .set_event_code(as_number(EventCodes::BlockData))
            .set_payload(payload)
            .build();

    UdpSocket::bind("127.0.0.1:0").unwrap()
        .send_to(&request, peer_address)
        .expect("Sending a request should be successful");
}