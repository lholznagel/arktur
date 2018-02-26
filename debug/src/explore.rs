use blockchain_hooks::{ApplicationState, as_enum, EventCodes, Hooks, HookRegister};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::ExploreNetworkPayload;

use clap::ArgMatches;
use futures_cpupool::CpuPool;

use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};
use std::sync::{Arc, Mutex};
use std::process::exit;
use std::{thread, time};

pub fn execute(hole_puncher: String, _: &ArgMatches) {
    let pool = CpuPool::new_num_cpus();
    let mut threads = Vec::new();

    let state = Arc::new(Mutex::new(ExploreState::new()));

    let hooks = Hooks::new()
        .set_explore_network(on_explore_network);

    let mut hook_notification = HookRegister::new(hooks, Arc::clone(&state))
        .get_notification();

    let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_status_code(StatusCodes::Ok)
        .build();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Binding an UdpSocket should be successful.");
    socket.send_to(&request, hole_puncher).expect("Sending a request should be successful");

    #[allow(unreachable_code)]
    let peer_sync = pool.spawn_fn(move || {
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

        let res: Result<bool, ()> = Ok(true);
        res
    });

    threads.push(peer_sync);

    thread::sleep(time::Duration::from_secs(30));
    threads.pop().unwrap().forget();

    let mut success = 0;
    let mut fail = 0;

    let state_lock = state.lock().expect("Locking the mutex should be successful.");
    for (address, value) in &state_lock.peers {
        if state_lock.peers.len() - 1 == value.len() {
            success!("Peer {} knows all peers", address);
            success += 1;
        } else {
            error!("Peer {} does not know all peers", address);
            fail += 1;
        }
    }

    info!("Success: {}, Fail: {}", success, fail);

    exit(0);
}

pub struct ExploreState {
    peers: HashMap<String, Vec<String>>
}

impl ExploreState {
    /// Creates a new empty instance of ExploreHandler
    pub fn new() -> Self {
        Self {
            peers: HashMap::new()
        }
    }
}

pub fn on_explore_network(state: ApplicationState<ExploreState>) {
    let message = BlockchainProtocol::<ExploreNetworkPayload>::from_bytes(&state.payload_buffer).expect("Parsing should be successful");
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");

    if !state_lock.peers.contains_key(&state.source) {
        state_lock.peers.insert(state.source, message.payload.addresses.clone());

        for address in message.payload.addresses {
            let request = BlockchainProtocol::<ExploreNetworkPayload>::new()
                .set_event_code(EventCodes::ExploreNetwork)
                .set_status_code(StatusCodes::Ok)
                .build();

            if !address.is_empty() && !state_lock.peers.contains_key(&address) {
                state.udp.send_to(&request, address.parse::<SocketAddr>().unwrap()).expect("Sending a request should be successful");
            }
        }
    }
}