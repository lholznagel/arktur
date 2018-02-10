use blockchain_hooks::{ApplicationState, EventCodes, Hooks};
use blockchain_protocol::payload::{Payload, RegisterAckPayload, ExploreNetworkPayload};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::status::StatusCodes;

pub struct StateHandler {
    peers: Vec<String>
}

impl StateHandler {
    pub fn new() -> Self {
        Self {
            peers: Vec::new()
        }
    }
}

pub struct HookHandler;

impl Hooks<StateHandler> for HookHandler {
    fn on_register_hole_puncher(&self, state: ApplicationState<StateHandler>) {
        let mut state_lock = state.state.lock().expect("Locking the mutex should be successful.");
        event!("New peer: {}", state.source);
        if state_lock.peers.is_empty() {
            sending!("ACK_REGISTER | NO_PEER");
            let answer = BlockchainProtocol::new()
                .set_event_code(EventCodes::RegisterHolePuncherAck)
                .set_status_code(StatusCodes::NoPeer)
                .set_payload(RegisterAckPayload::new())
                .build();
            state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
        } else {
            sending!("ACK_REGISTER | PEER");
            let answer = BlockchainProtocol::new()
                .set_event_code(EventCodes::RegisterHolePuncherAck)
                .set_status_code(StatusCodes::Ok)
                .set_payload(RegisterAckPayload::new().set_peers(state_lock.peers.clone()))
                .build();
            state.udp.send_to(&answer, state.source.clone()).expect("Sending a response should be successful");
        }

        state_lock.peers.push(state.source);
    }

    fn on_explore_network(&self, state: ApplicationState<StateHandler>) {
        debug!("Sending peers to debugger");
        let state_lock = state.state.lock().expect("Locking the mutex should be successful.");

        let answer = BlockchainProtocol::new()
            .set_event_code(EventCodes::ExploreNetwork)
            .set_status_code(StatusCodes::Ok)
            .set_payload(ExploreNetworkPayload::new().set_peers(state_lock.peers.clone()))
            .build();
        state.udp.send_to(&answer, state.source).expect("Sending a response should be successful");
    }

    fn on_ping(&self, _: ApplicationState<StateHandler>) {}
    fn on_pong(&self, _: ApplicationState<StateHandler>) {}
    fn on_register_hole_puncher_ack(&self, _: ApplicationState<StateHandler>) {}
    fn on_register_peer(&self, _: ApplicationState<StateHandler>) {}
    fn on_register_peer_ack(&self, _: ApplicationState<StateHandler>) {}
    fn on_data_for_block(&self, _: ApplicationState<StateHandler>) {}
    fn on_new_block(&self, _: ApplicationState<StateHandler>) {}
    fn on_possible_block(&self, _: ApplicationState<StateHandler>) {}
    fn on_validate_hash(&self, _: ApplicationState<StateHandler>) {}
    fn on_validated_hash(&self, _: ApplicationState<StateHandler>) {}
    fn on_found_block(&self, _: ApplicationState<StateHandler>) {}
}