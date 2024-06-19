use clap::{Parser, Subcommand};
use std::time::Instant;

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

    let start = Instant::now();

    match &cli.command {
        Commands::Init => ratchet::init(),
        Commands::Turn => ratchet::turn(),
        Commands::Check => ratchet::check(),
    }

    let duration = start.elapsed().as_secs_f32();
    println!("⚡Ratchet finished in {:.2}s", duration);
}
