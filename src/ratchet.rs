use regex::Regex;
use std::{cmp::Ordering, collections::BTreeMap, fs::read_to_string, path::Path, process};
use walkdir::WalkDir;

use crate::{
    config::{self, read_config, RATCHET_CONFIG},
    ratchet_file::{RatchetFile, RuleMap, RuleName, RATCHET_FILE},
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
    process_rules(config, file, false, false);
}

pub fn check(config: &String, file: &String) {
    println!("👀 Checking ratchet!\n");
    process_rules(config, file, true, false)
}

pub fn force(config: &String, file: &String) {
    println!("⛓️‍💥 Forcing ratchet!\n");
    process_rules(config, file, false, true);
}

fn process_rules(config_path: &String, file: &String, is_check: bool, is_force: bool) {
    let config = read_config(config_path);
    // HACK: Test comment to get it in the RATCHET_FILE file

    let previous_ratchet = RatchetFile::load(file);

    let mut rules_map: BTreeMap<RuleName, RuleMap> = BTreeMap::new();

    // TODO: Parallelize this someday
    config.rules.iter().for_each(|(key, value)| {
        let mut rule_map = BTreeMap::new();

        let regex = Regex::new(&value.regex).expect("Failed to compile regex");

        // TODO: Clean the regexes up
        let include_regex = value
            .include
            .as_ref()
            .map(|include| Regex::new(include).expect("Failed to compile include regex"));

        let exclude_regex = value
            .exclude
            .as_ref()
            .map(|exclude| Regex::new(exclude).expect("Failed to compile include regex"));

        for entry in WalkDir::new(".") {
            let entry = entry.unwrap();
            // If it's not a file, there's nothing to analyze. Keep going!
            if !entry.file_type().is_file() {
                continue;
            }

            let path_str = entry.path().to_string_lossy();

            // TODO: Better way to ignore well-known files
            if path_str.ends_with(RATCHET_FILE)
                || path_str.ends_with(RATCHET_CONFIG)
                || path_str.contains(".git")
            {
                continue;
            }

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

            let content = read_to_string(entry.path());
            if let Err(_e) = content {
                // println!("Failed to read file, continuing: {:?}", e);
                continue;
            }
            let content = content.unwrap();

            let matches: Vec<_> = regex.find_iter(&content).collect();
            for found in matches {
                let key = (entry.path().display().to_string(), "hash_me".to_string());
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

    let ratchet_file = RatchetFile {
        version: config.version,
        rules: rules_map,
    };

    let mut got_worse = false;

    // for each rule, see if it got better or worse than the previous
    for (rule, previous_rule_items) in &previous_ratchet.rules {
        let mut previous_rule_count = 0;
        let mut new_rule_count = 0;

        match ratchet_file.rules.get(rule) {
            Some(new_rule) => {
                for (key, value) in previous_rule_items {
                    match new_rule.get(key) {
                        Some(new_value) => {
                            previous_rule_count += value.len();
                            new_rule_count += new_value.len();
                        }
                        None => println!("Key: {:?} does not exist in the current file", key),
                    }
                }
            }
            None => println!("Rule {} does not exist in the current file", rule),
        }

        got_worse = new_rule_count > previous_rule_count;
        match new_rule_count.cmp(&previous_rule_count) {
            Ordering::Greater => {
                println!("❌ Rule {} got worse ({} new issues out of {} total)", rule, new_rule_count - previous_rule_count, new_rule_count);
            }
            Ordering::Less => {
                println!("🛠️ Rule {} improved ({} issues fixed out of {} total", rule, previous_rule_count - new_rule_count, new_rule_count);
            }
            Ordering::Equal => {
                println!("✔️ Rule {} did not change ({} total)", rule, new_rule_count);
            }
        }
    }

    // We don't want to update if we're just checking the state of the code or if things got worse
    // OR if we're forcing the update
    if !is_check && !got_worse || is_force {
        ratchet_file.save(file);
    }
    // If we're checking and things got worse, exist with an error for CI
    else if is_check && got_worse {
        process::exit(1);
    }
}
