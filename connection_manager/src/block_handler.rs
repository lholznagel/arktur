use blockchain_file::peers::KnownPeers;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use blockchain_protocol::payload::NewBlockPayload;

use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::time::Duration;

pub fn handle_block(udp: UdpSocket) {
    loop {
        let last_peers = KnownPeers::get_all();

        if last_peers.len() > 0 {
            let payload = NewBlockPayload::genesis().set_content(String::from("Some content"));

            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::NewBlock)
                .set_payload(payload)
                .build();

            for peer in last_peers {
                println!("{:?}", peer.get_socket());
                udp.send_to(
                    message.as_slice(),
                    peer.get_socket().parse::<SocketAddr>().unwrap(),
                ).unwrap();
            }
        }

        println!("Send");

        thread::sleep(Duration::from_secs(10));
    }
}