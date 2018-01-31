use blockchain_hooks::{EventCodes, Hooks, HookRegister};
use blockchain_network::udp_client::UdpClientBuilder;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::ExploreNetworkPayload;

use clap::ArgMatches;

use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::process::exit;

pub fn execute(hole_puncher: String, _: &ArgMatches) {
    let hook_notification = HookRegister::new()
        .set_hook(ExploreHandler::new())
        .get_notification();

    let udp_client = UdpClientBuilder::new().build(hook_notification);

    let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
            .set_event_code(EventCodes::ExploreNetwork)
            .set_status_code(StatusCodes::Ok)
            .build();

    udp_client.udp.send_to(&request, hole_puncher.parse::<SocketAddr>().unwrap()).expect("Sending a request should be successful");

    udp_client.listen();
}

/// Contains all hooks that the peer listens to
pub struct ExploreHandler {
    is_first_run: bool,
    peers: HashMap<String, Vec<String>>,
    peers_to_check: Vec<String>,
    repeats: u8
}

impl ExploreHandler {
    /// Creates a new empty instance of ExploreHandler
    pub fn new() -> Self {
        Self {
            is_first_run: true,
            peers: HashMap::new(),
            peers_to_check: Vec::new(),
            repeats: 0
        }
    }
}

impl Hooks for ExploreHandler {
    fn on_explore_network(&mut self, udp: &UdpSocket, payload_buffer: Vec<u8>, source: String) {
        let message = BlockchainProtocol::<ExploreNetworkPayload>::from_bytes(&payload_buffer).expect("Parsing should be successful");

        if !self.peers.contains_key(&source) {
            if self.is_first_run {
                self.is_first_run = false;
                self.peers_to_check = message.payload.addresses.clone();
            } else {
                self.peers.insert(source, message.payload.addresses.clone());
            }

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

            if self.repeats == self.peers_to_check.len() as u8 {
                let mut excluded = 0;
                let mut success = 0;
                let mut fail = 0;

                for address in &self.peers_to_check {
                    if !self.peers.contains_key(address) {
                        error!("No response from {}. Excluding", address);
                        excluded += 1;
                    }
                }

                for (address, value) in &self.peers {
                    if self.peers.len() - 1 == value.len() - excluded {
                        success!("Peer {} knows all peers", address);
                        success += 1;
                    } else {
                        error!("Peer {} does not know all peers", address);
                        fail += 1;
                    }
                }

                info!("Success: {}, Fail: {}, Excluded: {}", success, fail, excluded);

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