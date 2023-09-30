use clap::Args;
use core::config;
use core::converters;
use core::extractor;
use core::repo;
use spinners::{Spinner, Spinners};
use std::process::exit;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or_default();
    let mut spin = Spinner::new(Spinners::Line, "Cloning repository...".to_string());
    let mut git_repo = match repo::clone_repository(&repo, &conf) {
        Ok(r) => r,
        Err(err) => {
            println!("Error: {err}");
            exit(1);
        }
    };
    spin.stop_with_message(format!(
        "`{}` repository cloned successfully",
        git_repo.folder_path
    ));

    let mut spin = Spinner::new(Spinners::Line, "Extracting data...".to_string());
    let (extracted_data, _) = match extractor::extract(&mut git_repo) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to extract repository data: {err}");
            exit(1);
        }
    };
    spin.stop_with_message(format!(
        "Extraction completed checkout the `{}` generated.",
        git_repo.extracted_file_path
    ));

    let mut spin = Spinner::new(Spinners::Line, "Converting data...".to_string());
    let conv = converters::shmup::new();
    if let Err(err) = converters::convert(&mut git_repo, extracted_data, &conv) {
        println!("Error: failed to convert extracted data: {err}");
        exit(1);
    };
    spin.stop_with_message(format!(
        "Convertion completed checkout the `{}` generated.",
        git_repo.converted_file_path
    ));
}
