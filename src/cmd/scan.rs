use crate::config;
use clap::Args;
use std::fs;
use std::path;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repos: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repos = args.repos.clone().unwrap_or("".to_string());

    fetch_repository(repos, conf);
}

pub fn fetch_repository(repos: String, conf: config::Config) {
    //Create the temporary directory if it doesn't exist
    let path = path::Path::new(&conf.tmp_folder);
    if !path.exists() {
        fs::create_dir(&conf.tmp_folder).unwrap_or_else(|err| {
            println!("Failed to create `{}` folder: {}", conf.tmp_folder, err);
        });
        println!("`{}` Temporary folder was created", conf.tmp_folder);
    }
    println!("I will fetch the repository than store it's content");
}
