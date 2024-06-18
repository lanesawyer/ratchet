use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RatchetConfig {
    pub version: u8,
    // TODO: I don't think I like this structure, revisit
    pub rules: HashMap<String, RatchetRule>,
}

#[derive(Debug, Deserialize)]
pub struct RatchetRule {
    // TODO: Definitely revisit, not every rule is regex
    pub regex: String,
}

pub fn read_config() -> RatchetConfig {
    let contents =
        fs::read_to_string("ratchet.toml").expect("Something went wrong reading the file");

    let ratchet_config: RatchetConfig = toml::from_str(&contents).expect("Failed to deserialize");

    println!("{:?}", ratchet_config);
    ratchet_config
}
