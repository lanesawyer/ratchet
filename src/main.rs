use clap::{Parser, Subcommand};
use regex::Regex;
use ron::ser::PrettyConfig;
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
};
use walkdir::WalkDir;

use ratchet::{RatchetFile, RuleMap, RuleName};

mod config;
mod ratchet;

/// Ratchet is a tool to help you add new rules to your project over time
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new ratchet project
    Init,
    /// Turn the ratchet, updating the file if things are good and erroring if things got worse
    Turn,
    /// Check that no rules have been violated
    Check,
}

fn main() {
    let cli = Cli::parse();

    let config = config::read_config();
    // HACK: Test hack comment to get it in the ratchet.ron file
    print!("config: {:?}", config);

    let mut rules_map: HashMap<RuleName, RuleMap> = HashMap::new();

    match &cli.command {
        Commands::Init => {
            println!("initializing!");
        }
        Commands::Turn => {
            println!("turning!");

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
                version: 1,
                rules: rules_map,
            };
            let pretty_config = PrettyConfig::new()
                .depth_limit(3)
                .separate_tuple_members(true)
                .enumerate_arrays(true);
            let ron = ron::ser::to_string_pretty(&ratchet_file, pretty_config)
                .expect("Serialization failed");

            let mut file = File::create("ratchet.ron").expect("Failed to create file");
            file.write_all(ron.as_bytes())
                .expect("Failed to write to file");
        }
        Commands::Check => {
            println!("checking!");
        }
    }
}
