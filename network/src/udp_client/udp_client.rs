use std::net::{IpAddr, UdpSocket};
use std::str;

/// Stores all needed infomration about a udp client
pub struct UdpClient {
    /// open udp socket
    udp: UdpSocket,
    /// Handler for the register command
    register_handler: fn(String)
}

impl UdpClient {
    /// Creates a new UdpClient
    ///
    /// # Returns
    ///
    /// New instance of `UdpClient`
    pub fn new(udp: UdpSocket, register_handler: fn(String)) -> Self {
        UdpClient {
            udp: udp,
            register_handler: register_handler
        }
    }

    /// Gets the open socket connection
    ///
    /// # Return
    ///
    /// `UdpSocket` - Open UdpSocet
    ///
    /// # Example
    ///
    /// ```
    /// use blockchain_network::udp_client::{UdpClient, UdpClientBuilder};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build();
    ///
    /// let data = [0; 10];
    /// let address = "0.0.0.0:50000";
    /// udp_client.connection().send_to(&data, address);
    /// ```
    pub fn connection(self) -> UdpSocket {
        self.udp
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
    /// use blockchain_network::udp_client::{UdpClient, UdpClientBuilder};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build();
    ///
    /// println!("Port: {:?}", udp_client.port());
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
    /// use blockchain_network::udp_client::{UdpClient, UdpClientBuilder};
    ///
    /// let udp_client_builder = UdpClientBuilder::new();
    /// let udp_client = udp_client_builder.build();
    ///
    /// println!("IP-Address: {:?}", udp_client.ip());
    /// ```
    pub fn ip(self) -> IpAddr {
        self.udp.local_addr().unwrap().ip()
    }

    /// Listens to new UDP packages
    ///
    /// When a new command is identified the given callback is called
    ///
    /// This function is blocking!
    pub fn listen(self) {
        println!("starting listener");
        loop {
            let mut buffer = [0; 4096];

            match self.udp.recv_from(&mut buffer) {
                Ok((_, src)) => (self.register_handler)(String::from(str::from_utf8(&buffer).unwrap_or(""))),
                Err(e) => println!("Error: {:?}", e)
            }
        }
    }
}