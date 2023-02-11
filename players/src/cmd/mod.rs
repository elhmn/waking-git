pub mod shmup;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shmup will start the shmup game
    Shmup(shmup::RunArgs),
}

/// `play` launches a selection of games or simulators
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Shmup(args) => {
            shmup::run(args);
        }
    }
}
