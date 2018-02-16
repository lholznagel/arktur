use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::DataForBlockPayload;

use hooks::State;

pub fn on_data_for_block(state: ApplicationState<State>) {
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