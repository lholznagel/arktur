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

uri: 127.0.0.1:45001

secret_key: v+rETx4VtczK/QSvl9OBfJfgVPEdjNpquVUq/8GFmWo=
"#).unwrap();

    carina_core::init(config);
}
