pub mod play;
pub mod scan;

use crate::config;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// scans a git repository and extracts world data
    Scan(scan::RunArgs),
    /// play generate a playable/simulated world
    Play(play::RunArgs),
}

/// `Wake` git repository world generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

pub fn run(conf: config::Config) {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan(args) => {
            scan::run(args, conf);
        }
        Commands::Play(args) => {
            play::run(args, conf);
        }
    }
}
