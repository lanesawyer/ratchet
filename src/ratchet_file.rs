use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{read_to_string, File},
    io::Write,
};

pub const RATCHET_FILE_VERSION: u8 = 1;
pub const RATCHET_FILE: &str = "ratchet.ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetFile {
    pub version: u8,
    pub rules: BTreeMap<RuleName, RuleMap>,
}

pub type RuleName = String;
// TODO: Probably don't need file name and hash as the key
pub type RuleMap = BTreeMap<(FileName, FileHash), Problems>;

type FileName = String;
type FileHash = String;
type Problems = Vec<Problem>;

type Problem = (Start, End, MessageText, MessageHash);

type Start = usize;
type End = usize;
// TODO: The next two could be optional, rules like regex won't have a unique message
type MessageText = String;
type MessageHash = String;

impl RatchetFile {
    pub fn new() -> Self {
        RatchetFile {
            version: RATCHET_FILE_VERSION,
            rules: BTreeMap::new(),
        }
    }

    // TODO: Custom file location
    pub fn load(file: &String) -> Self {
        let contents = read_to_string(file);

        if contents.is_err() {
            println!("No ratchet results file found, evaluating initial baseline");
            return RatchetFile::new();
        }

        ron::de::from_str(&contents.unwrap()).expect("Failed to deserialize")
    }

    pub fn save(&self, file: &String) {
        let pretty_config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let ron = ron::ser::to_string_pretty(self, pretty_config).expect("Serialization failed");
        let ron = format!("{}\n", ron);

        let mut file = File::create(file).expect("Failed to create file");
        file.write_all(ron.as_bytes())
            .expect("Failed to write to file");
    }
}
