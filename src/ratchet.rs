use regex::Regex;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
};
use walkdir::WalkDir;

use crate::config::read_config;

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetFile {
    pub version: u8,
    pub rules: HashMap<RuleName, RuleMap>,
}

pub type RuleName = String;
// TODO: Probably don't need file name and hash as the key
pub type RuleMap = HashMap<(FileName, FileHash), Problems>;

type FileName = String;
type FileHash = String;
type Problems = Vec<Problem>;

type Problem = (Start, End, MessageText, MessageHash);

type Start = usize;
type End = usize;
// TODO: The next two could be optional, rules like regex won't have a unique message
type MessageText = String;
type MessageHash = String;

pub fn init() {
    println!("initializing ratchet!");
}

pub fn turn() {
    println!("turning ratchet!");

    let config = read_config();
    // HACK: Test hack comment to get it in the ratchet.ron file
    print!("config: {:?}", config);

    let mut rules_map: HashMap<RuleName, RuleMap> = HashMap::new();

    // TODO: Parallelize this someday
    config.rules.iter().for_each(|(key, value)| {
        let mut rule_map = HashMap::new();

        println!("Rule: {}", key);
        println!("Regexp: {}", value.regex);
        let regex = Regex::new(&value.regex).expect("Failed to compile regex");
        println!("Regex: {:?}", regex);
        for entry in WalkDir::new("src") {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                continue;
            }

            let content = read_to_string(entry.path()).expect("Failed to read file");
            let matches: Vec<_> = regex.find_iter(&content).collect();
            for found in matches {
                println!("Matched! {} {:?}", entry.path().display(), found);
                let key = (entry.path().display().to_string(), "hash_me".to_string());
                let value = (
                    found.start(),
                    found.end(),
                    regex.to_string(),
                    "hash_me".to_string(),
                );
                rule_map.entry(key).or_insert_with(Vec::new).push(value);
            }
            println!("{}", entry.path().display());
        }
        rules_map.insert(key.to_string(), rule_map);
    });

    let ratchet_file = RatchetFile {
        version: config.version,
        rules: rules_map,
    };
    // TODO: Deterministic order on the hashmap printing
    let pretty_config = PrettyConfig::new()
        .depth_limit(3)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    let ron =
        ron::ser::to_string_pretty(&ratchet_file, pretty_config).expect("Serialization failed");

    let mut file = File::create("ratchet.ron").expect("Failed to create file");
    file.write_all(ron.as_bytes())
        .expect("Failed to write to file");
}

pub fn check() {
    println!("checking ratchet!");
}
