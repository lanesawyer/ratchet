use std::{collections::HashMap, fs, io::Write};

use serde::{Deserialize, Serialize};

pub const RATCHET_CONFIG: &str = "ratchet.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetConfig {
    pub version: u8,
    // TODO: I don't think I like this structure, revisit
    pub rules: HashMap<String, RatchetRule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetRule {
    // TODO: Definitely revisit, not every rule is regex
    pub regex: String,
    // TODO: Consider storing a Regex that can serialize/deserialize
    pub include: Option<String>,
    // TODO: Consider storing a Regex that can serialize/deserialize
    pub exclude: Option<String>,
}

impl RatchetConfig {
    pub fn init() {
        let ratchet_config = RatchetConfig {
            version: 1,
            rules: HashMap::new(),
        };

        let toml = toml::to_string(&ratchet_config).expect("Failed to serialize");

        let toml = format!("{}\n", toml);

        let mut file = fs::File::create("ratchet.toml").expect("Failed to create file");
        file.write_all(toml.as_bytes())
            .expect("Failed to write to file");
    }
}

pub fn read_config() -> RatchetConfig {
    let contents =
        fs::read_to_string("ratchet.toml").expect("Something went wrong reading the file");

    let ratchet_config: RatchetConfig = toml::from_str(&contents).expect("Failed to deserialize");

    println!("{:?}", ratchet_config);
    ratchet_config
}
