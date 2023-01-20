use clap::Args;
use core::config;
use core::converters;
use core::extractor;
use core::repo;
use spinners::{Spinner, Spinners};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path;

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
            return println!("Error: {err}");
        }
    };
    spin.stop_with_message(format!(
        "`{}` repository cloned successfully",
        git_repo.folder_path
    ));

    let mut spin = Spinner::new(Spinners::Line, "Extracting data...".to_string());
    let extracted_data = match extract_data(&conf, &mut git_repo) {
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

    let mut spin = Spinner::new(Spinners::Dots, "Converting data...".to_string());
    let conv = converters::shmup::new();
    if let Err(err) = convert_data(&conf, &mut git_repo, extracted_data, &conv) {
        println!("Error: failed to convert extracted data: {err}");
    };
    spin.stop_with_message(format!(
        "Convertion completed checkout the `{}` generated.",
        git_repo.converted_file_path
    ));
}

pub fn extract_data(
    conf: &config::Config,
    git_repo: &mut repo::Repo,
) -> Result<extractor::Data, String> {
    let data = extractor::run(git_repo)?;
    let dest_folder = format!(
        "{}/{}/{}",
        conf.wake_path,
        config::SCANNER_FOLDER_NAME,
        git_repo.folder_name
    );
    let dest_path = format!("{}/{}", dest_folder, config::EXTRACTOR_FILE_NAME);
    git_repo.extracted_file_path = dest_path.clone();
    let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "".to_string());
    match store_json_data(json_data, dest_folder, &dest_path) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: failed to extract repository data: {err}"));
        }
    };

    Ok(data)
}

pub fn convert_data<Data: serde::Serialize>(
    conf: &config::Config,
    git_repo: &mut repo::Repo,
    extracted_data: extractor::Data,
    converter: &impl converters::Converter<Data>,
) -> Result<Data, String> {
    let data = converter.run(&extracted_data)?;
    let dest_folder = format!(
        "{}/{}/{}",
        conf.wake_path,
        config::SCANNER_FOLDER_NAME,
        git_repo.folder_name
    );
    let dest_path = format!(
        "{}/{}-{}",
        dest_folder,
        converter.name(),
        config::CONVERTER_FILE_NAME_PREFIX
    );
    git_repo.converted_file_path = dest_path.clone();
    let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "".to_string());
    match store_json_data(json_data, dest_folder, &dest_path) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: failed to convert repository data: {err}"));
        }
    };

    Ok(data)
}

pub fn store_json_data(data: String, dest_folder: String, dest_path: &String) -> Result<(), Error> {
    let path = path::Path::new(&dest_folder);
    if !path.exists() {
        fs::create_dir_all(&dest_folder)?
    }

    let mut file = File::create(dest_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
