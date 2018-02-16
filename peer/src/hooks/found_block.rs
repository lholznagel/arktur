use blockchain_file::blocks::Block;
use blockchain_hooks::ApplicationState;
use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::payload::FoundBlockPayload;

use hooks::State;

pub fn on_found_block(state: ApplicationState<State>) {
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