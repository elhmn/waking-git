use crate::config;
use crate::extractor;
use crate::repo;
use clap::Args;
use std::fs;
use std::path;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or("".to_string());

    let r = clone_repository(repo, conf);
    extractor::run(r);
}

pub fn clone_repository(repo: String, conf: config::Config) -> Option<repo::Repo> {
    //Create the temporary directory if it doesn't exist
    let path = path::Path::new(&conf.tmp_folder);
    if !path.exists() {
        match fs::create_dir(&conf.tmp_folder) {
            Ok(()) => {
                println!("`{}` Temporary folder was created", conf.tmp_folder);
            },
            Err(err) => {
                println!("Failed to create `{}` folder: {}", conf.tmp_folder, err);
                return None;
            }
        };
    }

    let r = match repo::new_repo_from_url(repo.to_string(), &conf.tmp_folder) {
        Ok(r) => r,
        Err(err) => {
            println!("Error: {}", err);
            return None;
        }
    };

    println!("`{}` repository cloned successfully", repo);
    return Some(r)
}
