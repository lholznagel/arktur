use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::SyncPeersPayload;

use hooks::State;

pub fn on_sync_peers(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<SyncPeersPayload>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");
    info!("Syncing peers.");

    {
        let mut state_lock = state.state.lock()
            .expect("Locking the mutex should be successful.");

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