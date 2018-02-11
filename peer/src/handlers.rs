use blockchain_file::blocks::Block;
use blockchain_hooks::{ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{Payload, DataForBlockPayload, FoundBlockPayload, PongPayload, RegisterPayload, RegisterAckPayload, NewBlockPayload, PossibleBlockPayload, ValidateHashPayload, ValidatedHashPayload, ExploreNetworkPayload};

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::collections::HashMap;

pub struct StateHandler {
    next_block: HashMap<String, String>,
    peers: Vec<String>
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            next_block: HashMap::new(),
            peers: Vec::new()
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
    event!("NEW_BLOCK {:?}", message.payload);

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

    debug!("Found hash! {:?}", hash);
    let answer = BlockchainProtocol::<PossibleBlockPayload>::new()
        .set_event_code(EventCodes::PossibleBlock)
        .set_payload(PossibleBlockPayload {
            content: message.payload.content,
            timestamp: message.payload.timestamp,
            index: message.payload.index,
            prev: message.payload.prev,
            nonce: nonce,
            hash: hash
        });
    sending!("POSSIBLE_BLOCK | {:?}", answer.payload);
    success!("Send block back.");
    state.udp.send_to(&answer.build(), state.source).expect("Sending a response should be successful");
}

pub fn on_validate_hash(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<ValidateHashPayload>::from_bytes(&state.payload_buffer);
    let message = message.unwrap();
    event!("VALIDATE_HASH {:?}", message.payload);

    let mut generated_block = String::from("");
    generated_block.push_str(&message.payload.content);
    generated_block.push_str(&message.payload.index.to_string());
    generated_block.push_str(&message.payload.timestamp.to_string());
    generated_block.push_str(&message.payload.prev);
    generated_block.push_str(&message.payload.nonce.to_string());

    let mut hasher = Sha3::sha3_256();
    hasher.input_str(generated_block.as_str());

    let mut answer = BlockchainProtocol::<ValidatedHashPayload>::new().set_event_code(EventCodes::ValidatedHash);
    answer.payload.index = message.payload.index;
    answer.payload.hash = hasher.result_str();
    state.udp.send_to(&answer.build(), state.source).expect("Sending a response should be successful");
}

pub fn on_found_block(state: ApplicationState<StateHandler>) {
    let message = BlockchainProtocol::<FoundBlockPayload>::from_bytes(&state.payload_buffer);
    let message = message.unwrap();
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