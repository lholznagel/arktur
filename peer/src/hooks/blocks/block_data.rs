use carina_hooks::ApplicationState;
use carina_protocol::Protocol;
use carina_protocol::payload::blocks::BlockData;

use hooks::State;

pub fn block_data(state: ApplicationState<State>) {
    let message = Protocol::<BlockData>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    let mut state_lock = state.state.lock()
        .expect("Locking the mutex should be successful.");

    if !state_lock.next_block.contains_key(&message.payload.unique_key) {
        state_lock.next_block.insert(message.payload.unique_key, message.payload.content);
        info!("New message for next block. Notifying peers");

        for (peer, _) in &state_lock.peers {
            state.udp.send_to(&state.payload_buffer, peer).expect("Sending using UDP should be successful.");
        }
    }
}