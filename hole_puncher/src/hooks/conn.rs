use blockchain_hooks::{as_number, ApplicationState, EventCodes};
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::HolePuncherConn;

use hooks::State;

pub fn on_hole_puncher_conn(state: ApplicationState<State>) {
    let message = BlockchainProtocol::<HolePuncherConn>::from_bytes(&state.payload_buffer)
        .expect("Parsing the protocol should be successful.");

    let payload = HolePuncherConn {
        address: state.source
    };

    let result = BlockchainProtocol::<HolePuncherConn>::new()
        .set_payload(payload)
        .set_event_code(as_number(EventCodes::HolePuncherConn))
        .build();

    state.udp.send_to(&result, message.payload.address)
        .expect("Sending using UDP should be successful.");
}