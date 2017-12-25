use blockchain_file::peers::KnownPeers;
use blockchain_protocol::BlockchainProtocol;
use blockchain_hooks::EventCodes;
use blockchain_protocol::payload::NewBlockPayload;

use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::time::Duration;

pub fn handle_block(udp: UdpSocket) {
    debug!("Waiting 30 seconds for peers to connect");
    thread::sleep(Duration::from_secs(30));

    loop {
        let last_peers = KnownPeers::get_all();

        if last_peers.len() > 2 {
            let payload = NewBlockPayload::genesis();

            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::NewBlock)
                .set_payload(payload)
                .build();

            for peer in last_peers {
                udp.send_to(
                    message.as_slice(),
                    peer.get_socket().parse::<SocketAddr>().unwrap(),
                ).unwrap();
            }

            debug!("Send next block");
        } else {
            info!("Not enough peers connected. Waiting.");
        }

        // for now send every 2.5 minutes
        // with debug mode enabled it takes some time :D
        thread::sleep(Duration::from_secs(150));
    }
}

pub fn possible_block_handler(_: SocketAddr, _: &UdpSocket, message: BlockchainProtocol<PossibleBlockPayload>) {
    event!(format!("POSSIBLE_BLOCK {:?}", message.payload));
}