pub mod key;

mod config;


use std::path::Path;
use key::VirtualKeySet;
use key::VirtualKey;
use key::bind_key_sets;

use crate::config::Config;
use crate::config::read_config;

fn main() {
    init_key_mappings();
    key::listen_event();
}

fn init_key_mappings() {
    let config = read_config(Path::new("./resources/config.toml"));
    for x in config.key_mappings {
        bind_key_sets(x.origin.keys.as_slice(), x.mapping.keys.as_slice())
    }
}

fn
