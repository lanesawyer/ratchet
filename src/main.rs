use clap::{Parser, Subcommand};
use std::time::Instant;

use config::RATCHET_CONFIG;

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
    Init {
        /// Path for creating config file, defaults to ratchet.toml in the current directory
        #[clap(long, short, default_value = RATCHET_CONFIG)]
        config: String,
    },
    /// Turn the ratchet, updating the file if things are good and erroring if things got worse
    Turn {
        /// Path to the config file to use, defaults to ratchet.toml in the current directory
        #[clap(long, short, default_value = RATCHET_CONFIG)]
        config: String,
    },
    /// Check that no rules have been violated
    Check {
        /// Path to the config file to use, defaults to ratchet.toml in the current directory
        #[clap(long, short, default_value = "ratchet.toml")]
        config: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();

    match &cli.command {
        Commands::Init { config } => ratchet::init(config),
        Commands::Turn { config } => ratchet::turn(config),
        Commands::Check { config } => ratchet::check(config),
    }

    let duration = start.elapsed().as_secs_f32();
    println!("⚡Ratchet finished in {:.2}s", duration);
}
