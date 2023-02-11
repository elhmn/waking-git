use crate::cmd::scan;
use clap::Args;
use core::config;
use core::converters;
use core::repo;
use spinners::{Spinner, Spinners};

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the type of player
    /// can be one of: [shmup, ...]
    #[clap(value_name = "PLAYER", index = 1)]
    player: Option<String>,

    /// the path to the repository we want to play
    #[clap(value_name = "REPOSITORY", index = 2)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or_default();
    let player = args.player.clone().unwrap_or_default();

    let mut spin = Spinner::new(Spinners::Line, "Cloning repository...".to_string());
    let mut git_repo = match repo::clone_repository(&repo, &conf) {
        Ok(r) => r,
        Err(err) => {
            return println!("Error: {err}");
        }
    };
    spin.stop_with_message(format!(
        "`{}` repository cloned successfully",
        git_repo.folder_path
    ));

    let mut spin = Spinner::new(Spinners::Line, "Extracting data...".to_string());
    let extracted_data = match scan::extract_data(&conf, &mut git_repo) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to extract repository data: {err}");
            return;
        }
    };
    spin.stop_with_message(format!(
        "Extraction completed checkout the `{}` generated.",
        git_repo.extracted_file_path
    ));

    let mut spin = Spinner::new(Spinners::Line, "Converting data...".to_string());
    let conv = converters::shmup::new();
    if let Err(err) = scan::convert_data(&conf, &mut git_repo, extracted_data, &conv) {
        println!("Error: failed to convert extracted data: {err}");
    };
    spin.stop_with_message(format!(
        "Convertion completed checkout the `{}` generated.",
        git_repo.converted_file_path
    ));

    println!("Running the player...");

    // check if we are running the binary for integration tests
    // because we don't want to open a window while running tests
    if std::env::var("WAKE_TEST_MODE").is_ok() {
        return;
    }

    if let Err(err) = core::exec::run_player(player, git_repo.converted_file_path) {
        println!("Error: failed to run the player: {err}");
    };
}
