use blockchain_protocol::BlockchainProtocol;
use blockchain_protocol::enums::events::EventCodes;
use std::net::{SocketAddr, UdpSocket};

pub fn ping_handler(source: SocketAddr, udp: &UdpSocket, _: BlockchainProtocol) {
    event!(format!("PING from peer {:?}", source.to_string()));
    sending!(format!("PONG to peer {:?}", source.to_string()));
    let answer = BlockchainProtocol::new().set_event_code(EventCodes::Pong).build();
    udp.send_to(answer.as_slice(), source).unwrap();
    success!(format!("Send PONG to {:?}", source.to_string()));
}

pub fn pong_handler(source: SocketAddr, _: &UdpSocket, _: BlockchainProtocol) {
    event!(format!("PONG from peer {:?}", source.to_string()));
}

pub fn peer_registering_handler(_: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol) {
    event!(format!("PEER_REGISTERING {:?}", message.data));
    sending!(format!("PING to new peer {:?}", message.data));
    let answer = BlockchainProtocol::new().set_event_code(EventCodes::Ping).build();
     udp.send_to(answer.as_slice(), message.data.parse::<SocketAddr>().unwrap()).unwrap();
    success!(format!("Send PING {:?}", message.data));
}

pub fn register_ack_handler(_: SocketAddr, udp: &UdpSocket, message: BlockchainProtocol) {
     if message.data == "NO_PEER" {
         error!("No peer");
     } else {
        sending!(format!("PING to peer {:?}", message.data));
        let answer = BlockchainProtocol::new().set_event_code(EventCodes::Ping).build();
        udp.send_to(answer.as_slice(), message.data.parse::<SocketAddr>().unwrap()).unwrap();
        success!(format!("Send PING to {:?}", message.data));
     }
}