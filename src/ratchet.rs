use regex::Regex;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};
use walkdir::WalkDir;

use crate::config::{self, read_config, RATCHET_CONFIG};

const RATCHET_FILE: &str = "ratchet.ron";

#[derive(Debug, Deserialize, Serialize)]
pub struct RatchetFile {
    pub version: u8,
    pub rules: HashMap<RuleName, RuleMap>,
}

impl RatchetFile {
    // TODO: Custom file location
    pub fn load() -> Self {
        let contents = read_to_string(RATCHET_FILE).expect("Failed to read file");
        ron::de::from_str(&contents).expect("Failed to deserialize")
    }

    // TODO: Custom file location
    pub fn save(&self) {
        // TODO: Deterministic order on the hashmap printing
        let pretty_config = PrettyConfig::new()
            .depth_limit(4)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let ron = ron::ser::to_string_pretty(self, pretty_config).expect("Serialization failed");
        let ron = format!("{}\n", ron);

        let mut file = File::create(RATCHET_FILE).expect("Failed to create file");
        file.write_all(ron.as_bytes())
            .expect("Failed to write to file");
    }
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

    let path = Path::new(RATCHET_CONFIG);
    if path.exists() {
        println!("Ratchet config already exists");
        return;
    }

    config::RatchetConfig::init();
}

pub fn turn() {
    println!("turning ratchet!");

    process_rules(false);
}

pub fn check() {
    println!("checking ratchet!");
    process_rules(true)
}

fn process_rules(is_check: bool) {
    let config = read_config();
    // HACK: Test comment to get it in the RATCHET_FILE file
    print!("config: {:?}", config);

    let previous_ratchet = RatchetFile::load();
    println!("Previous Ratchet: {:?}", previous_ratchet);

    let mut rules_map: HashMap<RuleName, RuleMap> = HashMap::new();

    // TODO: Parallelize this someday
    config.rules.iter().for_each(|(key, value)| {
        let mut rule_map = HashMap::new();

        println!("Rule: {}", key);
        println!("Regexp: {}", value.regex);
        let regex = Regex::new(&value.regex).expect("Failed to compile regex");
        println!("Regex: {:?}", regex);

        // TODO: Clean the regexes up
        let include_regex = value
            .include
            .as_ref()
            .map(|include| Regex::new(include).expect("Failed to compile include regex"));

        let exclude_regex = value
            .exclude
            .as_ref()
            .map(|exclude| Regex::new(exclude).expect("Failed to compile include regex"));

        for entry in WalkDir::new("src") {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {
                continue;
            }

            let path_str = entry.path().to_string_lossy();

            if include_regex.is_some() && !include_regex.as_ref().unwrap().is_match(&path_str) {
                println!(
                    "Skipping (not included): {} for {}",
                    entry.path().display(),
                    key
                );
                continue;
            }

            if exclude_regex.is_some() && exclude_regex.as_ref().unwrap().is_match(&path_str) {
                println!(
                    "Skipping (excluded): {} for {}",
                    entry.path().display(),
                    key
                );
                continue;
            }

            // TODO: Got error running on another codebase:
            // Failed to read file: Error { kind: InvalidData, message: "stream did not contain valid UTF-8" }
            let content = read_to_string(entry.path());
            if let Err(e) = content {
                println!("Failed to read file: {:?}", e);
                continue;
            }
            let content = content.unwrap();

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

    // for each rule, see if it got better or worse than the previous
    for (rule, previous_rule_items) in &previous_ratchet.rules {
        match ratchet_file.rules.get(rule) {
            Some(new_rule) => {
                for (key, value) in previous_rule_items {
                    match new_rule.get(key) {
                        Some(new_value) => match new_value.len().cmp(&value.len()) {
                            Ordering::Greater => {
                                println!("Rule {} has more items in the current file", rule);
                            }
                            Ordering::Less => {
                                println!("Rule {} has fewer items in the current file", rule);
                            }
                            Ordering::Equal => {
                                println!(
                                    "Rule {} has the same number of items in both files",
                                    rule
                                );
                            }
                        },
                        None => println!("Key: {:?} does not exist in the current file", key),
                    }
                    println!("Key: {:?}, Value: {:?}", key, value);
                }
            }
            None => println!("Rule {} does not exist in the current file", rule),
        }
    }

    if !is_check {
        ratchet_file.save();
    }
}
