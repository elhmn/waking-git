use git2::Repository;
use std::path;
use url::Url;

pub struct Repo {
    pub repo: Repository,
    pub folder_name: String,
}

pub fn new_repo_from_url(url: String, repo_storage: &String) -> Result<Repo, String> {
    //Validate url and extract repository name
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
        match Repository::open(dest_path) {
            Ok(git_repo) => git_repo,
            Err(err) => {
                return Err(format!("Failed to clone `{url}` repository: {err}"));
            }
        }
    };

    let repo = Repo {
        repo: git_repo,
        folder_name,
    };
    Ok(repo)
}
