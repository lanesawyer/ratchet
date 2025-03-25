use std::{collections::BTreeMap, fs, io::Write};

use serde::{Deserialize, Serialize};

use crate::{ratchet_file::RATCHET_FILE, rules::RatchetRule};

pub const CONFIG_VERSION: u8 = 1;
pub const RATCHET_CONFIG: &str = "ratchet.toml";

// TODO: What else should be considered well known?
pub const WELL_KNOWN_FILES: [&str; 4] = [
    // Ratchet specific
    RATCHET_FILE,
    RATCHET_CONFIG,
    // No need to look in the git folder!
    ".git",
    // Rust specific
    // TODO: What if they're not running from root directory
    "./target",
];

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetConfig {
    pub version: u8,
    // TODO: I don't think I like this structure, revisit
    pub rules: BTreeMap<String, RatchetRule>,
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
