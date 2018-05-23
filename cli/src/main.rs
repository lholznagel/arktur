extern crate carina_core;
extern crate log;
extern crate loggify;

use carina_core::Config;

fn main() {
    loggify::Loggify::init_with_level(log::Level::Debug).unwrap();

    let config = Config::from_str(r#"---
socket: /tmp/carina.sock

peers: ./example_peers.yml

storage: ./block_data

uri: 0.0.0.0:45000

secret_key: W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY=
"#).unwrap();
    
    carina_core::init(config);
}
