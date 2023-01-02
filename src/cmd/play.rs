use super::scan;
use crate::config;
use crate::converters;
use crate::players::shmup;
use clap::Args;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to play
    #[clap(value_name = "REPOSITORY", index = 1)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or_default();

    let git_repo = match scan::clone_repository(&repo, &conf) {
        Ok(r) => {
            println!("`{}` repository cloned successfully", repo);
            r
        }
        Err(err) => {
            return println!("Error: {}", err);
        }
    };

    let extracted_data = match scan::extract_data(&conf, &git_repo) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to extract repository data: {}", err);
            return;
        }
    };

    let conv = converters::shmup::new();
    let converted_data = match scan::convert_data(&conf, &git_repo, extracted_data, &conv) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to convert extracted data: {}", err);
            return;
        }
    };

    println!("Running the player...");

    // check if we are running the binary for integration tests
    // because we don't want to open a window while running tests
    if std::env::var("WAKE_TEST_MODE").is_ok() {
        return;
    }

    shmup::run(converted_data);
}
