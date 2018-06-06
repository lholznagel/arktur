extern crate carina_core;
#[macro_use]
extern crate log;
extern crate loggify;

mod events;

use carina_core::{CarinaConfigBuilder, Config, Events};
use std::sync::Arc;

fn main() {
    loggify::Loggify::init_with_level(log::Level::Debug).unwrap();

    let config = Config::from_str(r#"---
socket: /tmp/carina.sock

peers: ./example_peers.yml

storage: ./block_data

uri: 127.0.0.1:45001

secret_key: v+rETx4VtczK/QSvl9OBfJfgVPEdjNpquVUq/8GFmWo=
"#).unwrap();

    let ping_event = events::Ping{};
    let pong_event = events::Pong{};

    let carina_config_builder = CarinaConfigBuilder::new()
        .add_event(Events::Ping, Arc::new(ping_event))
        .add_event(Events::Pong, Arc::new(pong_event))
        .set_config(config);
    carina_core::init(carina_config_builder);
}
