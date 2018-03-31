/// Configuration for the peer
#[derive(Clone, Debug)]
pub struct Config {
    /// Configuration for the hole puncher
    pub hole_puncher: HolePuncher,
    /// Port the peer should listen on
    pub port: u16,
    /// Storage for the blocks
    pub storage: String,
}

/// Hole puncher configuration
#[derive(Clone, Debug)]
pub struct HolePuncher {
    /// Host address of the hole puncher
    pub host: String,
    /// Port of the hole puncher
    pub port: u16,
}

impl Config {
    /// Creates a new config instance
    pub fn new() -> Self {
        Self {
            hole_puncher: HolePuncher::new(),
            port: 0,
            storage: String::from("block_data")
        }
    }
}

impl HolePuncher {
    /// Creates a new hole puncher instance
    pub fn new() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: 50000
        }
    }

    /// Gets the address of the hole puncher
    pub fn address(&self) -> String {
        let mut address = String::new();
        address.push_str(&self.host);
        address.push_str(":");
        address.push_str(&self.port.to_string());
        address
    }
}