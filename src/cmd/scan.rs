use crate::config;
use crate::converters;
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
pub const EXTRACTOR_FILE_NAME: &str = "extracted.json";
pub const CONVERTER_FILE_NAME_PREFIX: &str = "converted.json";

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or_default();

    let git_repo = match clone_repository(&repo, &conf) {
        Ok(r) => {
            println!("`{repo}` repository cloned successfully");
            r
        }
        Err(err) => {
            return println!("Error: {err}");
        }
    };

    let extracted_data = match extract_data(&conf, &git_repo) {
        Ok(d) => d,
        Err(err) => {
            println!("Error: failed to extract repository data: {err}");
            return;
        }
    };

    let conv = converters::shmup::new();
    if let Err(err) = convert_data(&conf, &git_repo, extracted_data, &conv) {
        println!("Error: failed to convert extracted data: {err}");
    };
}

pub fn extract_data(
    conf: &config::Config,
    git_repo: &repo::Repo,
) -> Result<extractor::Data, String> {
    let data = extractor::run(git_repo)?;
    let dest_folder = format!(
        "{}/{}/{}",
        conf.wake_path, SCANNER_FOLDER_NAME, git_repo.folder_name
    );
    let dest_path = format!("{dest_folder}/{EXTRACTOR_FILE_NAME}");
    let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "".to_string());
    match store_json_data(json_data, dest_folder, dest_path.clone()) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: failed to extract repository data: {err}"));
        }
    };

    println!("Extraction completed checkout the `{dest_path}` generated.");
    Ok(data)
}

pub fn convert_data<Data: serde::Serialize>(
    conf: &config::Config,
    git_repo: &repo::Repo,
    extracted_data: extractor::Data,
    converter: &impl converters::Converter<Data>,
) -> Result<Data, String> {
    let data = converter.run(&extracted_data)?;
    let dest_folder = format!(
        "{}/{}/{}",
        conf.wake_path, SCANNER_FOLDER_NAME, git_repo.folder_name
    );
    let dest_path = format!(
        "{}/{}-{}",
        dest_folder,
        converter.name(),
        CONVERTER_FILE_NAME_PREFIX
    );
    let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "".to_string());
    match store_json_data(json_data, dest_folder, dest_path.clone()) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: failed to convert extracted data: {err}"));
        }
    };

    println!("Convertion completed checkout the `{dest_path}` generated.");
    Ok(data)
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
            return Err(format!("Error: {err}"));
        }
    };
    Ok(r)
}

pub fn store_json_data(data: String, dest_folder: String, dest_path: String) -> Result<(), Error> {
    let path = path::Path::new(&dest_folder);
    if !path.exists() {
        fs::create_dir_all(&dest_folder)?
    }

    let mut file = File::create(dest_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
