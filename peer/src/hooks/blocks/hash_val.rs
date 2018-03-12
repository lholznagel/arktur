use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::blocks::{HashVal, HashValAck};

use hooks::State;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn hash_val(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<HashVal>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("Validating hash.");

    let mut generated_block = String::from("");
    generated_block.push_str(&message.payload.content);
    generated_block.push_str(&message.payload.index.to_string());
    generated_block.push_str(&message.payload.timestamp.to_string());
    generated_block.push_str(&message.payload.prev);
    generated_block.push_str(&message.payload.nonce.to_string());

    let mut hasher = Sha3::sha3_256();
    hasher.input_str(generated_block.as_str());

    let mut message = BlockchainProtocol::<HashValAck>::new().set_event_code(as_number(EventCodes::HashValAck));
    message.payload.index = message.payload.index;
    message.payload.hash = hasher.result_str();
    let message = message.build();

    let state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");
    for (peer, _) in state_lock.peers.clone() {
        state.udp.send_to(message.as_slice(), peer).expect("Sending using UDP should be successful.");
    }
}