use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::peers::GetPeersAckPayload;

use hooks::State;

pub fn get_peers_ack(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<GetPeersAckPayload>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("Syncing peers.");

    {
        let mut state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");

        for new_peer in message.payload.peers {
            if !state_lock.peers.contains_key(&new_peer) {
                state_lock.peers.insert(new_peer, 0);
            }
        }
    }
}