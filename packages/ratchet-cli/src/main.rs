use clap::{Parser, Subcommand};
use ratchet::{RATCHET_CONFIG, RATCHET_FILE};
use std::time::Instant;

/// Ratchet is a tool to help you add new rules to your project over time
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// TODO: Check if there's a way to make the subcommands more DRY
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
        /// Path for location of ratchet file, defaults to ratchet.ron in the current directory
        #[clap(long, short, default_value = RATCHET_FILE)]
        file: String,
    },
    /// Check that no rules have been violated
    Check {
        /// Path to the config file to use, defaults to ratchet.toml in the current directory
        #[clap(long, short, default_value = "ratchet.toml")]
        config: String,
        /// Path for location of ratchet file, defaults to ratchet.ron in the current directory
        #[clap(long, short, default_value = RATCHET_FILE)]
        file: String,
    },
    /// Force the results to be updated, even if they got worse
    Force {
        /// Path to the config file to use, defaults to ratchet.toml in the current directory
        #[clap(long, short, default_value = "ratchet.toml")]
        config: String,
        /// Path for location of ratchet file, defaults to ratchet.ron in the current directory
        #[clap(long, short, default_value = RATCHET_FILE)]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();

    match &cli.command {
        Commands::Init { config } => ratchet::init(config),
        Commands::Turn { config, file } => ratchet::turn(config, file),
        Commands::Check { config, file } => ratchet::check(config, file),
        Commands::Force { config, file } => ratchet::force(config, file),
    }

    let duration = start.elapsed().as_secs_f32();
    println!("\n⚡Ratchet finished in {:.2}s", duration);
}
