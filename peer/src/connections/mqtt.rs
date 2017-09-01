use mqtt::control::variable_header::ConnectReturnCode;
use mqtt::packet::{ConnectPacket, VariablePacket, SubscribePacket, ConnackPacket, Packet};
use mqtt::qos::QualityOfService;
use mqtt::topic_filter::TopicFilter;
use mqtt::{Encodable, Decodable};
use std::io::Write;
use std::net::TcpStream;
use std::str;

pub fn init() {
    let mut stream = TcpStream::connect("localhost:1883").unwrap();

    let mut connection = ConnectPacket::new("MQTT", "blockchain");
    connection.set_clean_session(true);
    connection.set_keep_alive(30);

    let mut buffer = Vec::new();
    connection.encode(&mut buffer).unwrap();
    stream.write_all(&buffer[..]).unwrap();

    // check if connected
    let connack = ConnackPacket::decode(&mut stream).unwrap();
    if connack.connect_return_code() != ConnectReturnCode::ConnectionAccepted {
        panic!(
            "Failed to connect to server, return code {:?}",
            connack.connect_return_code()
        );
    }

    // listen for pers that connect
    let topic_filter = TopicFilter::new("/peer/register").unwrap();
    let quality_of_service = QualityOfService::Level0;

    let subscribe = SubscribePacket::new(10, vec![(topic_filter, quality_of_service)]);
    let mut buffer = Vec::new();
    subscribe.encode(&mut buffer).unwrap();
    stream.write_all(&buffer[..]).unwrap();

    loop {
        let packet = match VariablePacket::decode(&mut stream) {
            Ok(pk) => pk,
            Err(err) => {
                println!("Error in receiving packet {:?}", err);
                continue;
            }
        };

        if let VariablePacket::SubackPacket(ref ack) = packet {
            if ack.packet_identifier() != 10 {
                panic!("SUBACK packet identifier not match");
            }

            break;
        }
    }

    loop {
        let packet = match VariablePacket::decode(&mut stream) {
            Ok(pk) => pk,
            Err(err) => {
                println!("Error in receiving packet {}", err);
                continue;
            }
        };

        match packet {
            VariablePacket::PingrespPacket(..) => {
                println!("Ping from broker");
            }
            VariablePacket::PublishPacket(ref publ) => {
                let msg = match str::from_utf8(&publ.payload()[..]) {
                    Ok(msg) => msg,
                    Err(err) => {
                        println!("Failed to decode publish message {:?}", err);
                        continue;
                    }
                };

                println!("Message ({}): {}", publ.topic_name(), msg);
            }
            _ => {}
        };
    }
}