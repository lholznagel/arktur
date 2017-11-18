use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use blockchain_protocol::enums::status::StatusCodes;
use blockchain_protocol::payload::{PingPayload, PongPayload, RegisterAckPayload, NewBlockPayload, PeerRegisteringPayload};
use std::net::{SocketAddr, UdpSocket};

pub fn ping_handler(source: SocketAddr, udp: &UdpSocket, _: BlockchainProtocol<PingPayload>) {
    event!(format!("PING from peer {:?}", source.to_string()));
    sending!(format!("PONG to peer {:?}", source.to_string()));
    let answer = BlockchainProtocol::<PongPayload>::new().set_event_code(EventCodes::Pong).build();
    udp.send_to(answer.as_slice(), source).unwrap();
    success!(format!("Send PONG to {:?}", source.to_string()));
}

pub fn pong_handler(source: SocketAddr, _: &UdpSocket, _: BlockchainProtocol<PongPayload>) {
    event!(format!("PONG from peer {:?}", source.to_string()));
}

pub fn register_ack_handler(_: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol<RegisterAckPayload>) {
     if message.status_code == StatusCodes::NoPeer {
         error!("No peer");
     } else {
        sending!(format!("PING to peer {:?}", message.payload));
        let answer = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
        udp.send_to(answer.as_slice(), message.payload.addr.parse::<SocketAddr>().unwrap()).unwrap();
        success!(format!("Send PING to {:?}", message.payload));
     }
}

pub fn peer_registering_handler(_: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol<PeerRegisteringPayload>) {
    event!(format!("PEER_REGISTERING {:?}", message.payload));
    sending!(format!("PING to new peer {:?}", message.payload));
    let answer = BlockchainProtocol::<PingPayload>::new().set_event_code(EventCodes::Ping).build();
     udp.send_to(answer.as_slice(), message.payload.addr.parse::<SocketAddr>().unwrap()).unwrap();
    success!(format!("Send PING {:?}", message.payload));
}

pub fn new_block_handler(_: SocketAddr, _: &UdpSocket, message: BlockchainProtocol<NewBlockPayload>) {
    event!(format!("NEW_BLOCK {:?}", message.payload));
}