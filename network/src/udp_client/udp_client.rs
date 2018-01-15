use blockchain_protocol::BlockchainProtocol;
use blockchain_hooks::{as_enum, EventCodes};
use blockchain_protocol::payload::*;
use blockchain_hooks::HookNotification;
use std::net::{IpAddr, UdpSocket, SocketAddr};

/// Stores all needed information about a udp client
pub struct UdpClient {
    /// open udp socket
    udp: UdpSocket,
    /// Contains all registered event handlers
    register: HookNotification
}

impl UdpClient {
    /// Creates a new UdpClient
    ///
    /// # Returns
    ///
    /// New instance of `UdpClient`
    pub fn new(udp: UdpSocket, register: HookNotification) -> Self {
        UdpClient {
            udp: udp,
            register: register
        }
    }

    /// Notifies the hole puncher one th given address
    ///
    /// # Parameter
    ///
    /// `address` - Address of the hole puncher server
    ///
    /// # Return
    ///
    /// Instance of the `UdpClient`
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    /// use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    ///
    /// # fn main() {
    /// 
    ///     let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    ///     UdpClientBuilder::new().set_port(50000).build(HookRegister::new().get_notification()).notify_hole_puncher(address, String::from("PeerName"));
    /// # }
    /// ```
    pub fn notify_hole_puncher(self, address: SocketAddr, name: String) -> Self {
        let mut payload = RegisterPayload::new();
        payload.name = name;

        let message = BlockchainProtocol::<RegisterPayload>::new()
            .set_event_code(EventCodes::Register)
            .set_payload(payload)
            .build();

        self.udp.send_to(message.as_slice(), address).unwrap();
        self
    }

    /// Gets the open socket connection
    ///
    /// # Return
    ///
    /// `UdpSocket` - Open UdpSocket
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    /// 
    ///     let data = [0; 10];
    ///     let address = "0.0.0.0:50000";
    ///     udp_client.connection().send_to(&data, address).unwrap();
    /// # }
    /// ```
    pub fn connection(&self) -> UdpSocket {
        self.udp.try_clone().unwrap()
    }

    /// Gets the current port
    ///
    /// # Return
    ///
    /// `u16` - Port the socket runs on
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    /// 
    ///     println!("Port: {:?}", udp_client.port());
    /// # }
    /// ```
    pub fn port(self) -> u16 {
        self.udp.local_addr().unwrap().port()
    }

    /// Gets the current IP-Address
    ///
    /// # Return
    ///
    /// `IpAddr` - IP-Address the socket runs on
    ///
    /// # Example
    ///
    /// ```
    /// extern crate blockchain_hooks;
    /// extern crate blockchain_network;
    ///
    /// use blockchain_hooks::HookRegister;
    /// use blockchain_network::udp_client::UdpClientBuilder;
    ///
    /// # fn main() {
    ///     let udp_client_builder = UdpClientBuilder::new();
    ///     let udp_client = udp_client_builder.build(HookRegister::new().get_notification());
    /// 
    ///     println!("IP-Address: {:?}", udp_client.ip());
    /// # }
    /// ```
    pub fn ip(self) -> IpAddr {
        self.udp.local_addr().unwrap().ip()
    }

    /// Listens to new UDP packages
    ///
    /// When a new event is identified the given callback is called
    ///
    /// This function is blocking!
    pub fn listen(mut self) -> Self {
        loop {
            let mut buffer = [0; 1024];

            match self.udp.recv_from(&mut buffer) {
                Ok((bytes, source)) => {
                    let mut updated_buffer = Vec::new();
                    for i in 0..bytes {
                        updated_buffer.push(buffer[i])
                    }

                    self.register.notify(&self.udp, as_enum(updated_buffer[0]), updated_buffer, source.to_string());
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}