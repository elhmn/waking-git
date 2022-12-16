use crate::config;
use crate::extractor;
use crate::repo;
use clap::Args;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path;

pub const REPOS_FOLDER_NAME: &str = "repos";
pub const SCANNER_FOLDER_NAME: &str = "scanner";
pub const SCANNER_FILE_NAME: &str = "extracted.json";

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or("".to_string());

    let git_repo = match clone_repository(&repo, &conf) {
        Ok(r) => {
            println!("`{}` repository cloned successfully", repo);
            r
        }
        Err(err) => {
            return println!("Error: {}", err);
        }
    };

    let data = match extractor::run(&git_repo) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to extract repository data: {}", err);
            return;
        }
    };

    let dest_folder = format!(
        "{}/{}/{}",
        conf.wake_path, SCANNER_FOLDER_NAME, git_repo.folder_name
    );
    let dest_path = format!("{}/{}", dest_folder, SCANNER_FILE_NAME);
    let json_data = serde_json::to_string(&data).unwrap_or("".to_string());
    match store_scanned_data(json_data, dest_folder, dest_path.clone()) {
        Ok(_) => (),
        Err(err) => {
            println!("Error: failed to extract repository data: {}", err);
            return;
        }
    };

    println!("Scan completed checkout the `{}` generated.", dest_path);
}

pub fn store_scanned_data(
    data: String,
    dest_folder: String,
    dest_path: String,
) -> Result<(), Error> {
    let path = path::Path::new(&dest_folder);
    if !path.exists() {
        fs::create_dir_all(&dest_folder)?
    }

    let mut file = File::create(dest_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}

pub fn clone_repository(repo: &String, conf: &config::Config) -> Result<repo::Repo, String> {
    //Create the temporary directory if it doesn't exist
    let path = path::Path::new(&conf.wake_path);
    if !path.exists() {
        match fs::create_dir(&conf.wake_path) {
            Ok(()) => {
                println!("`{}` Temporary folder was created", conf.wake_path);
            }
            Err(err) => {
                return Err(format!(
                    "Failed to create `{}` folder: {}",
                    conf.wake_path, err
                ));
            }
        };
    }

    let storage_folder = format!("{}/{}", conf.wake_path, REPOS_FOLDER_NAME);
    let r = match repo::new_repo_from_url(repo.to_string(), &storage_folder) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!("Error: {}", err));
        }
    };
    return Ok(r);
}
