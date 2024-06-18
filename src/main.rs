use clap::{Parser, Subcommand};

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

    match &cli.command {
        Commands::Init => ratchet::init(),
        Commands::Turn => ratchet::turn(),
        Commands::Check => ratchet::check(),
    }
}
