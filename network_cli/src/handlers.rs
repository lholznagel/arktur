use std::net::{SocketAddr, UdpSocket};

pub fn ping_handler(source: SocketAddr, udp: &UdpSocket, _: &str) {
    event!(format!("PING from peer {:?}", source.to_string()));
    sending!(format!("PONG to peer {:?}", source.to_string()));
    udp.send_to(b"PONG |", source).unwrap();
    success!("Send PONG");
}

pub fn pong_handler(source: SocketAddr, _: &UdpSocket, _: &str) {
    event!(format!("PONG from peer {:?}", source.to_string()));
}

pub fn peer_registering_handler(_: SocketAddr, udp: &UdpSocket, message: &str) {
    event!(format!("PEER_REGISTERING {:?}", message.replace("PEER_REGISTERING | ", "")));
    sending!(format!("PING to new peer {:?}", message.replace("PEER_REGISTERING | ", "")));
    udp.send_to(b"PING |", message.replace("PEER_REGISTERING | ", "").parse::<SocketAddr>().unwrap()).unwrap();
    success!(format!("Send PING {:?}", message.replace("PEER_REGISTERING | ", "")));
}

pub fn register_ack_handler(_: SocketAddr, udp: &UdpSocket, message: &str) {
     if message.replace("ACK_REGISTER | ", "") == "NO_PEER" {
         error!("No peer");
     } else {
        sending!(format!("PONG to peer {:?}", message.replace("ACK_REGISTER | ", "")));
        udp.send_to(b"PING |", message.replace("ACK_REGISTER | ", "").parse::<SocketAddr>().unwrap()).unwrap();
        success!(format!("Send PING to {:?}", message.replace("ACK_REGISTER | ", "")));
     }
}