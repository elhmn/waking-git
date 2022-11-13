use crate::config;
use clap::Args;
use git2::Repository;
use std::fs;
use std::io::Error;
use std::path;
use std::process;
use url::{Host, Position, Url};

#[derive(Args, Debug)]
pub struct RunArgs {
    /// the path to the repository we want to scan
    #[clap(value_name = "REPOSITORY", index = 1)]
    repository: Option<String>,
}

pub fn run(args: &RunArgs, conf: config::Config) {
    let repo = args.repository.clone().unwrap_or("".to_string());

    clone_repository(repo, conf);
}

pub fn clone_repository(repo: String, conf: config::Config) {
    //Create the temporary directory if it doesn't exist
    let path = path::Path::new(&conf.tmp_folder);
    if !path.exists() {
        fs::create_dir(&conf.tmp_folder).unwrap_or_else(|err| {
            println!("Failed to create `{}` folder: {}", conf.tmp_folder, err);
        });
        println!("`{}` Temporary folder was created", conf.tmp_folder);
    }

    //Validate url and extract repository name
    let repo_name: String;
    let repo_owner: String;
    let mut host_name: String;
    let url = Url::parse(&repo);
    match url {
        Err(err) => {
            println!("Failed to parse `{}` repository url: {}", repo, err);
            return;
        }
        Ok(url) => {
            //Check that the repo is a url
            if url.scheme() != "https" {
                println!("Failed to fetch the repository: Repository not a https url");
                return;
            }

            //Extract repository name and owner
            let path_segments: Vec<&str> = url.path().split("/").collect();
            if path_segments.len() <= 2 {
                println!("Failed to parse repository owner and name from `{}`", url);
                return;
            }

            repo_owner = path_segments[1].to_string();
            repo_name = path_segments[2].to_string();
            let host_str = url.host_str();
            match host_str {
                Some(h) => host_name = format!("{}-", h),
                None => host_name = "".to_string(),
            }

            host_name = host_name.replace(".", "-").to_string();
        }
    }

    //Clone the repository if it doesn't exist on disk
    let repo_path = format!(
        "{}/{}{}-{}",
        conf.tmp_folder, host_name, repo_owner, repo_name
    );
    let _git_repo: Repository;
    let path = path::Path::new(&repo_path);
    if !path.exists() {
        _git_repo = match Repository::clone(repo.as_str(), &repo_path) {
            Ok(git_repo) => git_repo,
            Err(err) => {
                println!("Failed to clone `{}` repository: {}", repo, err);
                return;
            }
        };
    } else {
        println!("`{}` git repository already on disk", repo_path);
        return;
    }

    println!("`{}` repository cloned successfully", repo_path);
}
