use regex::Regex;
use std::{collections::BTreeMap, fs::read_to_string, path::Path, process};
use walkdir::WalkDir;

use crate::{
    config::{self, read_config, WELL_KNOWN_FILES},
    ratchet_file::{RatchetFile, RuleMap, RuleName},
};

pub fn init(config: &String) {
    println!("🎬 Initializing ratchet!\n");

    let path = Path::new(config);
    if path.exists() {
        println!("Ratchet config already exists at {}", config);
        return;
    }

    config::RatchetConfig::init();
}

pub fn turn(config: &String, file: &String) {
    println!("⚙️ Turning ratchet!\n");
    let (got_worse, new_ratchet) = process_rules(config, file);

    if !got_worse {
        new_ratchet.save(file);
    }
}

pub fn check(config: &String, file: &String) {
    println!("👀 Checking ratchet!\n");
    let (got_worse, _) = process_rules(config, file);

    if got_worse {
        process::exit(1);
    }
}

pub fn force(config: &String, file: &String) {
    println!("⛓️‍💥 Forcing ratchet!\n");
    let (_, new_ratchet) = process_rules(config, file);

    // We don't care if things got better or worse, we're saving regardless!
    new_ratchet.save(file);
}

fn process_rules(config_path: &String, file: &String) -> (bool, RatchetFile) {
    let config = read_config(config_path);
    // HACK: Test comment to get it in the RATCHET_FILE file

    let previous_ratchet = RatchetFile::load(file);

    let mut rules_map: BTreeMap<RuleName, RuleMap> = BTreeMap::new();

    // TODO: Parallelize this someday
    config.rules.iter().for_each(|(key, value)| {
        let mut rule_map = BTreeMap::new();

        let regex = Regex::new(&value.regex).expect("Failed to compile regex");

        // TODO: Clean the regexes up
        let include_regexes: Option<Vec<Regex>> = value.include.as_ref().map(|include| {
            include
                .iter()
                .map(|i| Regex::new(i).expect("Failed to compile include regex"))
                .collect()
        });

        let exclude_regexes: Option<Vec<Regex>> = value.exclude.as_ref().map(|exclude| {
            exclude
                .iter()
                .map(|e| Regex::new(e).expect("Failed to compile include regex"))
                .collect()
        });

        for entry in WalkDir::new(".") {
            let entry = entry.unwrap();
            // If it's not a file, there's nothing to analyze. Keep going!
            if !entry.file_type().is_file() {
                continue;
            }

            let path_str = entry.path().to_string_lossy();
            if WELL_KNOWN_FILES
                .iter()
                .any(|&pattern| path_str.ends_with(pattern) || path_str.contains(pattern))
            {
                continue;
            }

            if include_regexes.is_some()
                && !include_regexes
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|r| r.is_match(&path_str))
            {
                println!(
                    "Skipping (not included): {} for {}",
                    entry.path().display(),
                    key
                );
                continue;
            }

            if exclude_regexes.is_some()
                && exclude_regexes
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|r| r.is_match(&path_str))
            {
                println!(
                    "Skipping (excluded): {} for {}",
                    entry.path().display(),
                    key
                );
                continue;
            }

            let content = read_to_string(entry.path());
            if let Err(_e) = content {
                // println!("Failed to read file, continuing: {:?}", e);
                continue;
            }
            let content = content.unwrap();

            let matches: Vec<_> = regex.find_iter(&content).collect();
            for found in matches {
                // TODO: The actual hashing, but the compare function needs fixing first
                // let file_hash = seahash::hash(content.as_bytes());
                let key = (entry.path().display().to_string(), 1234);
                let value = (
                    found.start(),
                    found.end(),
                    regex.to_string(),
                    "hash_me".to_string(),
                );
                rule_map.entry(key).or_insert_with(Vec::new).push(value);
            }
        }
        rules_map.insert(key.to_string(), rule_map);
    });

    let new_ratchet = RatchetFile {
        version: config.version,
        rules: rules_map,
    };

    let got_worse = previous_ratchet.compare(&new_ratchet);

    (got_worse, new_ratchet)
}
