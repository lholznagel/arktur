extern crate carina_core;

use carina_core::Config;

fn main() {
    let config = Config::from_str(r#"---
socket: /tmp/carina.sock

peers: ./example_peers.yml

storage: ./block_data

uri: 0.0.0.0:45000

secret_key: W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY=
"#).unwrap();
    /*let config = Config::new(
        "./tmp/carina.sock".to_string(),
        "./example_peers.yml".to_string(),
        "./block_data".to_string(),
        "0.0.0.0:45000".to_string(),
        "W8TAQuFECexfADKJik6WBrh4G5qFaOhzX2eBZFIV8kY=".to_string()
    ).unwrap();*/
    
    println!("{:#?}", config);
}
