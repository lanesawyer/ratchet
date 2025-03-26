use std::{collections::BTreeMap, fs::read_to_string, path::Path, process};
use walkdir::WalkDir;

use crate::{
    config::{self, WELL_KNOWN_FILES, read_config},
    ratchet_file::{RatchetFile, RuleMap, RuleName},
    rules::rule::Rule,
    utils::{to_normalized_file_contents, to_normalized_path},
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
        let mut rule_map: RuleMap = BTreeMap::new();

        for entry in WalkDir::new(".") {
            let entry = entry.unwrap();
            // If it's not a file, there's nothing to analyze. Keep going!
            if !entry.file_type().is_file() {
                continue;
            }

            let os_path = entry.path();
            let path_str = to_normalized_path(os_path);
            if WELL_KNOWN_FILES
                .iter()
                .any(|&pattern| path_str.ends_with(pattern) || path_str.contains(pattern))
            {
                continue;
            }

            if !value.analyze_file(&path_str) {
                println!("Skipping: {} for {}", os_path.display(), key);
                continue;
            }

            let content = read_to_string(os_path);
            if let Err(_e) = content {
                // println!("Failed to read file, continuing: {:?}", e);
                continue;
            }

            let content = content.unwrap();
            let content = to_normalized_file_contents(&content);

            let problems = value.check(&path_str, &content);
            if problems.is_empty() {
                continue;
            }

            let file_hash = seahash::hash(content.as_bytes());
            rule_map.insert((path_str, file_hash), problems);
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
