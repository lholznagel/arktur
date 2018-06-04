extern crate carina_core;
#[macro_use]
extern crate log;
extern crate loggify;

mod ping;

use carina_core::{Config, Events, StateBuilder};
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

    let ping_event_1 = ping::Ping{};
    let ping_event_2 = ping::Ping2{};
    let ping_event_3 = ping::Ping3{};

    let state_builder = StateBuilder::new()
        .add_event(Events::A, Arc::new(ping_event_1))
        .add_event(Events::B, Arc::new(ping_event_2))
        .add_event(Events::C, Arc::new(ping_event_3))
        .set_config(config);
    carina_core::init(state_builder);
}
