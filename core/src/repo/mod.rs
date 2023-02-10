use crate::config;
use git2::Repository;
use std::fs;
use std::path;
use url::Url;

pub struct Repo {
    pub repo: Repository,
    /// The slugged name of the git repository
    pub folder_name: String,

    /// The absolute path of the git repository location on disk
    pub folder_path: String,

    /// The absolute path of the scanner folder location on disk
    pub scanner_path: String,

    /// the absolute path of the extracted.json file generared by the extractor
    pub extracted_file_path: String,

    /// the absolute path of the converted.json file generared by the converter
    pub converted_file_path: String,
}

pub fn clone_repository(repo: &String, conf: &config::Config) -> Result<Repo, String> {
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

    let r = match new_repo_from_url(repo.to_string(), conf) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!("Error: {err}"));
        }
    };

    Ok(r)
}

pub fn new_repo_from_url(url: String, conf: &config::Config) -> Result<Repo, String> {
    let repo_storage = conf.storage_path.to_owned();
    let repo_name: String;
    let repo_owner: String;
    let mut host_name: String;
    let p_url = Url::parse(&url);
    match p_url {
        Err(err) => {
            return Err(format!("Failed to parse `{url}` repository url: {err}"));
        }
        Ok(p_url) => {
            //Check that the repo is a url
            if p_url.scheme() != "https" {
                return Err(
                    "Failed to fetch the repository: Repository not a https url".to_string()
                );
            }

            //Extract repository name and owner
            let path_segments: Vec<&str> = p_url.path().split('/').collect();
            if path_segments.len() <= 2 {
                return Err(format!(
                    "Failed to parse repository owner and name from `{p_url}`"
                ));
            }

            repo_owner = path_segments[1].to_string();
            repo_name = path_segments[2].to_string();
            match p_url.host_str() {
                Some(h) => host_name = format!("{h}-"),
                None => host_name = "".to_string(),
            }

            host_name = host_name.replace('.', "-");
        }
    }

    //Clone the repository if it doesn't exist on disk
    let folder_name = format!("{host_name}{repo_owner}-{repo_name}");
    let dest_path = format!("{repo_storage}/{folder_name}");
    let path = path::Path::new(&dest_path);
    let git_repo: Repository = if !path.exists() {
        match Repository::clone(url.as_str(), &dest_path) {
            Ok(git_repo) => git_repo,
            Err(err) => {
                return Err(format!("Failed to clone `{url}` repository: {err}"));
            }
        }
    } else {
        match Repository::open(&dest_path) {
            Ok(git_repo) => git_repo,
            Err(err) => {
                return Err(format!("Failed to clone `{url}` repository: {err}"));
            }
        }
    };
    let scanner_path = format!(
        "{}/{}/{}",
        conf.wake_path,
        config::SCANNER_FOLDER_NAME,
        folder_name
    );

    let repo = Repo {
        repo: git_repo,
        folder_name,
        folder_path: dest_path,
        scanner_path,
        extracted_file_path: "".to_string(),
        converted_file_path: "".to_string(),
    };

    Ok(repo)
}
