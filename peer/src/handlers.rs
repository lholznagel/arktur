use blockchain_file::blocks::Block;
use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, DataForBlockPayload, FoundBlockPayload, PongPayload, RegisterPayload, RegisterAckPayload, NewBlockPayload, PossibleBlockPayload, ValidateHashPayload, ValidatedHashPayload, ExploreNetworkPayload, SyncPeersPayload};

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::collections::HashMap;

pub struct StateHandler {
    current_block: FoundBlockPayload,
    next_block: HashMap<String, String>,
    /// all peers this peer is connected to
    pub peers: Vec<String>,
    hashes: Vec<String>,
    is_calculating: bool
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            current_block: FoundBlockPayload::new(),
            next_block: HashMap::new(),
            peers: Vec::new(),
            hashes: Vec::new(),
            is_calculating: false
        }
    }
}

pub fn on_ping(state: ApplicationState<StateHandler>) {
    event!("PING from peer {:?}", state.source);
    sending!("PONG to peer {:?}", state.source);
    let answer = BlockchainProtocol::<PongPayload>::new().set_event_code(EventCodes::Pong).build();
    state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    success!("Send PONG to {:?}", state.source);
}

pub fn on_pong(state: ApplicationState<StateHandler>) {
    event!("PONG from peer {:?}", state.source);
}

pub fn on_register_hole_puncher_ack(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&state.payload_buffer).unwrap();
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("ACK_REGISTER {:?}", message);

    if message.status_code == StatusCodes::NoPeer {
        info!("No peer registered at the hole puncher");
    } else {
        sending!("REGISTER to peer {:?}", message.payload);

        for address in message.payload.addresses {
            let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
            state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
            success!("Send REGISTER_PEER to {:?}", address);

            if !state_lock.peers.contains(&address) {
                state_lock.peers.push(address);
            }
        }
    }
}

pub fn on_register_peer(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<RegisterPayload>::from_bytes(&state.payload_buffer).unwrap();
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("ACK_REGISTER {:?}", message);

    if state_lock.peers.is_empty() {
        sending!("ACK_REGISTER | NO_PEER");
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterPeerAck)
            .set_status_code(StatusCodes::NoPeer)
            .set_payload(RegisterAckPayload::new())
            .build();
        state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    } else {
        sending!("ACK_REGISTER | PEER");
        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::RegisterPeerAck)
            .set_status_code(StatusCodes::Ok)
            .set_payload(RegisterAckPayload::new().set_peers(state_lock.peers.clone()))
            .build();
        state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
    }

    state_lock.peers.push(state.source);
    debug!("REGISTER: {}", state_lock.peers.len());
}

pub fn on_register_peer_ack(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<RegisterAckPayload>::from_bytes(&state.payload_buffer).expect("Parsing should be successful");
    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
    event!("ACK_REGISTER {:?}", message);

    if message.status_code == StatusCodes::NoPeer {
        info!("No peer from other peer");
    } else {
        for address in message.payload.addresses {
            if !state_lock.peers.contains(&address) {
                let result = BlockchainProtocol::<RegisterPayload>::new().set_event_code(EventCodes::RegisterPeer).build();
                state.udp.send_to(&result, address.clone()).expect("Sending a response should be successful");
                state_lock.peers.push(address.clone());
                success!("Send REGISTER_PEER to {:?}", address);
            } else {
                debug!("Peer already known");
            }
        }
    }
}

pub fn on_data_for_block(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<DataForBlockPayload>::from_bytes(&state.payload_buffer).expect("Parsing should be successful");
    event!("DATA_FOR_BLOCK {:?}", message);

    let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");

    debug!("{}", state_lock.next_block.contains_key(&message.payload.unique_key));
    if !state_lock.next_block.contains_key(&message.payload.unique_key) {
        state_lock.next_block.insert(message.payload.unique_key, message.payload.content);
        info!("Added new message.");

        for peer in &state_lock.peers {
            state.udp.send_to(&state.payload_buffer, peer).expect("Sending should be successful");
        }
    }
}

pub fn on_new_block(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<NewBlockPayload>::from_bytes(&state.payload_buffer).unwrap();
    {
        let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        if state_lock.is_calculating {
            return;
        } else {
            event!("NEW_BLOCK {:?}", message.payload);
            state_lock.is_calculating = true;
        }
    }

    let hash;
    let mut nonce = 0;

    loop {
        let mut generated_block = String::from("");
        generated_block.push_str(&message.payload.content);
        generated_block.push_str(&message.payload.index.to_string());
        generated_block.push_str(&message.payload.timestamp.to_string());
        generated_block.push_str(&message.payload.prev);
        generated_block.push_str(&nonce.to_string());

        let mut hasher = Sha3::sha3_256();
        hasher.input_str(generated_block.as_str());
        let hex = hasher.result_str();

        if message.payload.sign_key == &hex[..message.payload.sign_key.len()] {
            hash = hex.clone();
            break;
        } else {
            nonce += 1;
        }
    }

    {
        let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        state_lock.is_calculating = false;
        state_lock.current_block = FoundBlockPayload {
            content: message.payload.content.clone(),
            timestamp: message.payload.timestamp.clone(),
            index: message.payload.index.clone(),
            prev: message.payload.prev.clone(),
            nonce: nonce.clone(),
            hash: hash.clone()
        }
    }

    debug!("Found hash! {:?}", hash);
    let message = BlockchainProtocol::<PossibleBlockPayload>::new()
        .set_event_code(EventCodes::PossibleBlock)
        .set_payload(PossibleBlockPayload {
            content: message.payload.content,
            timestamp: message.payload.timestamp,
            index: message.payload.index,
            prev: message.payload.prev,
            nonce: nonce,
            hash: hash
        })
        .build();

    success!("Send block back.");
    
    let state_lock = state.state.lock().expect("Locking should be successful");
    for peer in state_lock.peers.clone() {
        state.udp.send_to(message.as_slice(), peer).expect("Sending a UDP message should be successful");
    }
}

pub fn on_validate_hash(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<ValidateHashPayload>::from_bytes(&state.payload_buffer).unwrap();
    event!("VALIDATE_HASH {:?}", message.payload);

    let mut generated_block = String::from("");
    generated_block.push_str(&message.payload.content);
    generated_block.push_str(&message.payload.index.to_string());
    generated_block.push_str(&message.payload.timestamp.to_string());
    generated_block.push_str(&message.payload.prev);
    generated_block.push_str(&message.payload.nonce.to_string());

    let mut hasher = Sha3::sha3_256();
    hasher.input_str(generated_block.as_str());

    let mut message = BlockchainProtocol::<ValidatedHashPayload>::new().set_event_code(EventCodes::ValidatedHash);
    message.payload.index = message.payload.index;
    message.payload.hash = hasher.result_str();
    let message = message.build();

    let state_lock = state.state.lock().expect("Locking should be successful");
    for peer in state_lock.peers.clone() {
        state.udp.send_to(message.as_slice(), peer).expect("Sending a UDP message should be successful");
    }
}

pub fn on_found_block(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<FoundBlockPayload>::from_bytes(&state.payload_buffer).expect("Parsing the protocol should be successful");
    event!("FOUND_BLOCK {:?}", message.payload);

    Block::init();
    let mut block = Block::new();
    block.index = message.payload.index;
    block.content = message.payload.content;
    block.timestamp = message.payload.timestamp;
    block.nonce = message.payload.nonce;
    block.prev = message.payload.prev;
    block.hash = message.payload.hash;
    block.save();
}

pub fn on_explore_network(state: ApplicationState<StateHandler>) {
    debug!("Sending peers to debugger");
    let state_lock = state.state.lock().expect("Locking should be successful");

    let answer = BlockchainProtocol::new()
        .set_event_code(EventCodes::ExploreNetwork)
        .set_status_code(StatusCodes::Ok)
        .set_payload(ExploreNetworkPayload::new().set_peers(state_lock.peers.clone()))
        .build();
    state.udp.send_to(&answer, state.source).expect("Sending a response should be successful");
}

pub fn on_sync_peers(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<SyncPeersPayload>::from_bytes(&state.payload_buffer).expect("Parsing the protocol should be successful");

    {
        let mut state_lock = state.state.lock().expect("Locking should be successful");

        for new_peer in message.payload.peers {
            let mut is_peer_known = false;

            for peer in state_lock.peers.clone() {
                if peer == new_peer {
                    is_peer_known = true;
                }
            }

            if !is_peer_known {
                state_lock.peers.push(new_peer);
            }
        }
    }
}

pub fn on_possible_block(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<PossibleBlockPayload>::from_bytes(&state.payload_buffer).expect("Parsing the protocol should be successful");

    event!("POSSIBLE_BLOCK | {:?}", message);

    let payload = ValidateHashPayload {
        content: message.payload.content,
        index: message.payload.index,
        nonce: message.payload.nonce,
        prev: message.payload.prev,
        timestamp: message.payload.timestamp
    };

    let message = BlockchainProtocol::new()
        .set_event_code(EventCodes::ValidateHash)
        .set_payload(payload)
        .build();

    let state_lock = state.state.lock().expect("Locking should be successful");
    for peer in state_lock.peers.clone() {
        state.udp.send_to(message.as_slice(), peer).expect("Sending a UDP message should be successful");
    }
}

pub fn on_validated_hash(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<ValidatedHashPayload>::from_bytes(&state.payload_buffer).expect("Parsing the protocol should be successful");
    let mut state_lock = state.state.lock().expect("Locking should be successful");
    event!("VALIDATED_HASH | {:?}", message);

    state_lock.hashes.push(message.payload.hash);

    if state_lock.hashes.len() == state_lock.peers.len() {
        let mut hashes = HashMap::new();

        for hash in state_lock.hashes.clone() {
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

        state_lock.hashes = Vec::new();
        debug!("Hash {} for block: {:?}", result.0, state_lock.current_block);

        state_lock.current_block.hash = result.0;

        let mut payload = FoundBlockPayload::new();
        payload.content = state_lock.current_block.content.clone();
        payload.index = state_lock.current_block.index;
        payload.nonce = state_lock.current_block.nonce;
        payload.prev = state_lock.current_block.prev.clone();
        payload.timestamp = state_lock.current_block.timestamp;
        payload.hash = state_lock.current_block.hash.clone();

        let message = BlockchainProtocol::new()
            .set_event_code(EventCodes::FoundBlock)
            .set_payload(payload)
            .build();

        for peer in state_lock.peers.clone() {
            state.udp.send_to(message.as_slice(), peer).unwrap();
        }
    }
}