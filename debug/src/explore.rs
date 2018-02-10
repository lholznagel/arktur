use blockchain_hooks::{ApplicationState, as_enum, EventCodes, Hooks, HookRegister};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::ExploreNetworkPayload;

use clap::ArgMatches;

use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::sync::{Arc, Mutex};
use std::process::exit;

pub fn execute(hole_puncher: String, _: &ArgMatches) {
    let mut hook_notification = HookRegister::new(Box::new(ExploreHandler), Arc::new(Mutex::new(ExploreState::new())))
        .get_notification();

    let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_status_code(StatusCodes::Ok)
        .build();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Binding an UdpSocket should be successful.");
    socket.send_to(&request, hole_puncher).expect("Sending a request should be successful");

    loop {
        let mut buffer = [0; 65535];

        match socket.recv_from(&mut buffer) {
            Ok((bytes, source)) => {
                let mut updated_buffer = Vec::new();
                for i in 0..bytes {
                    updated_buffer.push(buffer[i])
                }

                let socket_clone = socket.try_clone().expect("Cloning the socket should be successful.");
                hook_notification.notify(socket_clone, as_enum(updated_buffer[0]), updated_buffer, source.to_string());
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

pub struct ExploreState {
    is_first_run: bool,
    peers: HashMap<String, Vec<String>>,
    peers_to_check: Vec<String>,
    repeats: u8
}

impl ExploreState {
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

/// Contains all hooks that the peer listens to
pub struct ExploreHandler;

impl Hooks<ExploreState> for ExploreHandler {
    fn on_explore_network(&self, state: ApplicationState<ExploreState>) {
        let message = BlockchainProtocol::<ExploreNetworkPayload>::from_bytes(&state.payload_buffer).expect("Parsing should be successful");
        let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");

        if !state_lock.peers.contains_key(&state.source) {
            if state_lock.is_first_run {
                state_lock.is_first_run = false;
                state_lock.peers_to_check = message.payload.addresses.clone();
            } else {
                state_lock.peers.insert(state.source, message.payload.addresses.clone());
            }

            for address in message.payload.addresses {
                let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
                    .set_event_code(EventCodes::ExploreNetwork)
                    .set_status_code(StatusCodes::Ok)
                    .build();

                if !address.is_empty() && !state_lock.peers.contains_key(&address) {
                    state.udp.send_to(&request, address.parse::<SocketAddr>().unwrap()).expect("Sending a request should be successful");
                }
            }
        } else {
            state_lock.repeats += 1;

            if state_lock.repeats == state_lock.peers_to_check.len() as u8 {
                let mut excluded = 0;
                let mut success = 0;
                let mut fail = 0;

                for address in &state_lock.peers_to_check {
                    if !state_lock.peers.contains_key(address) {
                        error!("No response from {}. Excluding", address);
                        excluded += 1;
                    }
                }

                for (address, value) in &state_lock.peers {
                    if state_lock.peers.len() - 1 == value.len() - excluded {
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

    fn on_ping(&self, _: ApplicationState<ExploreState>) {}
    fn on_pong(&self, _: ApplicationState<ExploreState>) {}
    fn on_register_hole_puncher_ack(&self, _: ApplicationState<ExploreState>) {}
    fn on_register_peer(&self, _: ApplicationState<ExploreState>) {}
    fn on_register_peer_ack(&self, _: ApplicationState<ExploreState>) {}
    fn on_data_for_block(&self, _: ApplicationState<ExploreState>) {}
    fn on_new_block(&self, _: ApplicationState<ExploreState>) {}
    fn on_validate_hash(&self, _: ApplicationState<ExploreState>) {}
    fn on_found_block(&self, _: ApplicationState<ExploreState>) {}
    fn on_register_hole_puncher(&self, _: ApplicationState<ExploreState>) {}
    fn on_possible_block(&self, _: ApplicationState<ExploreState>) {}
    fn on_validated_hash(&self, _: ApplicationState<ExploreState>) {}
}