use blockchain_hooks::{EventCodes, Hooks};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, ExploreNetworkPayload};

use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::process::exit;

/// Contains all hooks that the peer listens to
pub struct HookHandler {
    peers: HashMap<String, Vec<String>>,
    repeats: u8
}

impl HookHandler {
    /// Creates a new empty instance of HookHandler
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
            repeats: 0
        }
    }
}

impl Hooks for HookHandler {
    fn on_explore_network(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<ExploreNetworkPayload>::from_bytes(&payload_buffer).expect("Parsing should be successful");

        if !self.peers.contains_key(&source) {
            info!("Peers: {:?}", message.payload.addresses);
            self.peers.insert(source, message.payload.addresses.clone());

            for address in message.payload.addresses {
                let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
                    .set_event_code(EventCodes::ExploreNetwork)
                    .set_status_code(StatusCodes::Ok)
                    .build();

                if !address.is_empty() && !self.peers.contains_key(&address) {
                    udp.send_to(&request, address.parse::<SocketAddr>().unwrap()).expect("Sending a request should be successful");
                }
            }
        } else {
            self.repeats += 1;

            if self.repeats == 10 {
                for (key, value) in &self.peers {
                    let mut combined_value = String::from("");

                    for current in value {
                        combined_value.push_str(&current);
                        combined_value.push_str(";");
                    }

                    println!("{};{}", key, combined_value);
                }

                exit(0);
            }
        }
    }

    fn on_ping(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_pong(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_hole_puncher_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_peer_ack(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_new_block(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validate_hash(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_found_block(&self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_register_hole_puncher(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_possible_block(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
    fn on_validated_hash(&mut self, _: &UdpSocket, _: Vec<u8>, _: String) {}
}