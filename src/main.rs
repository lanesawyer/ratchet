use std::{collections::HashMap, fs::File, io::Write};

use clap::{Parser, Subcommand};
use ratchet::{RatchetFile, RatchetItem};

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

    let mut test_map = HashMap::new();
    test_map.insert(
        ("test".to_string(), "test".to_string()),
        (
            "test_file".to_string(),
            "189831839".to_string(),
            vec![(1, 1, "This is the worst problem I've ever seen!".to_string())],
        ),
    );
    let ratchet_file = RatchetFile { items: test_map };

    match &cli.command {
        Commands::Init => {
            println!("initializing!");
        }
        Commands::Turn => {
            println!("turning!");
            let ron = ron::ser::to_string(&ratchet_file).expect("Serialization failed");

            let mut file = File::create("ratchet.ron").expect("Failed to create file");
            file.write_all(ron.as_bytes())
                .expect("Failed to write to file");
        }
        Commands::Check => {
            println!("checking!");
        }
    }
}
