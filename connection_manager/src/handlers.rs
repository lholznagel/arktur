use blockchain_hooks::Hooks;
use blockchain_protocol::payload::{FoundBlockPayload, NewBlockPayload, Payload, PossibleBlockPayload, ValidateHashPayload, ValidatedHashPayload};
use blockchain_hooks::EventCodes;
use blockchain_protocol::BlockchainProtocol;

use std::net::{UdpSocket, SocketAddr};
use std::collections::HashMap;
use time::get_time;

pub struct HookHandlers {
    connected_peers_addr: Vec<String>,
    current_block: PossibleBlockPayload,
    hashes: Vec<String>,
    validation_in_progress: bool,
    last_block_time: i64
}

impl HookHandlers {
    pub fn new() -> Self {
        Self {
            connected_peers_addr: Vec::new(),
            current_block: PossibleBlockPayload::new(),
            hashes: Vec::new(),
            validation_in_progress: false,
            last_block_time: 0
        }
    }

    fn send_next_block(&mut self, udp: UdpSocket) {
        let payload = NewBlockPayload::block(self.current_block.index + 1, self.current_block.hash.clone());
        self.last_block_time = payload.timestamp;
        self.validation_in_progress = false;

        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::NewBlock)
            .set_payload(payload)
            .build();

        for peer in self.connected_peers_addr.clone() {
            udp.send_to(
                message.as_slice(),
                peer.parse::<SocketAddr>().unwrap(),
            ).unwrap();
        }
    }
}

impl Hooks for HookHandlers {
    fn on_validated_hash(&mut self, udp: UdpSocket, payload_buffer: Vec<u8>, _: String) {
        let message = BlockchainProtocol::<ValidatedHashPayload>::from_bytes(&payload_buffer);
        let message = message.unwrap();
        event!("VALIDATED_HASH | {:?}", message);

        if message.payload.index == self.current_block.index {
            self.hashes.push(message.payload.hash);
        }

        if self.hashes.len() == self.connected_peers_addr.len() {
            let mut hashes = HashMap::new();

            for hash in self.hashes.clone() {
                let updated_value = match hashes.get(&hash) {
                    Some(current_val)   => current_val + 1,
                    None                => 1
                };

                hashes.insert(hash, updated_value);
            }

            let mut result: (String, u64) = (String::from(""), 0);
            for (key, value) in hashes {
                if result.1 == 0 || value > result.1 {
                    result.0 = key;
                    result.1 = value;
                }
            }

            self.hashes = Vec::new();
            debug!("Hash {} for block: {:?}", result.0, self.current_block);

            self.current_block.hash = result.0;

            let mut payload = FoundBlockPayload::new();
            payload.content = self.current_block.content.clone();
            payload.index = self.current_block.index;
            payload.nonce = self.current_block.nonce;
            payload.prev = self.current_block.prev.clone();
            payload.timestamp = self.current_block.timestamp;
            payload.hash = self.current_block.hash.clone();

            let message = BlockchainProtocol::new()
                .set_event_code(EventCodes::FoundBlock)
                .set_payload(payload)
                .build();

            for peer in self.connected_peers_addr.clone() {
                udp.send_to(message.as_slice(), peer.parse::<SocketAddr>().unwrap()).unwrap();
            }

            loop {
                // for now every 2 minutes
                if get_time().sec - self.last_block_time >= 120 {
                    self.send_next_block(udp);
                    break;
                }
            }
        }
    }
}
