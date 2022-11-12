pub mod scan;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// scans a git repository and extracts world data
    Scan(scan::RunArgs),
}

/// `Wake` git repository world generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan(args) => {
            scan::run(args);
        }
    }
}
