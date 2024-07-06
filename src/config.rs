use std::{collections::BTreeMap, fs, io::Write};

use serde::{Deserialize, Serialize};

use crate::ratchet_file::RATCHET_FILE;

pub const CONFIG_VERSION: u8 = 1;
pub const RATCHET_CONFIG: &str = "ratchet.toml";

// TODO: What else should be considered well known?
pub const WELL_KNOWN_FILES: [&str; 3] = [RATCHET_FILE, RATCHET_CONFIG, ".git"];

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetConfig {
    pub version: u8,
    // TODO: I don't think I like this structure, revisit
    pub rules: BTreeMap<String, RatchetRule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetRule {
    // TODO: Definitely revisit, not every rule is regex
    pub regex: String,
    // TODO: Consider storing a Regex that can serialize/deserialize
    // TODO: Make an array of strings
    pub include: Option<Vec<String>>,
    // TODO: Consider storing a Regex that can serialize/deserialize
    // TODO: Make an array of strings
    pub exclude: Option<Vec<String>>,
}

impl RatchetConfig {
    pub fn new() -> Self {
        RatchetConfig {
            version: CONFIG_VERSION,
            rules: BTreeMap::new(),
        }
    }

    pub fn init() {
        let ratchet_config = RatchetConfig::new();

        let toml = toml::to_string(&ratchet_config).expect("Failed to serialize");
        let toml = format!("{}\n", toml);

        let mut file = fs::File::create(RATCHET_CONFIG).expect("Failed to create file");
        file.write_all(toml.as_bytes())
            .expect("Failed to write to file");
    }
}

pub fn read_config(config_path: &String) -> RatchetConfig {
    let contents = fs::read_to_string(config_path).expect("Something went wrong reading the file");

    let ratchet_config: RatchetConfig = toml::from_str(&contents).expect("Failed to deserialize");

    ratchet_config
}
