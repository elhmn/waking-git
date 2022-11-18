use git2::Repository;
use url::Url;
use std::path;

pub fn new_repo_from_url(url: String, tmp_folder: &String) -> Result<Repo, String> {
    //Validate url and extract repository name
    let repo_name: String;
    let repo_owner: String;
    let mut host_name: String;
    let p_url = Url::parse(&url);
    match p_url {
        Err(err) => {
            return Err(format!("Failed to parse `{}` repository url: {}", url, err).to_owned());
        }
        Ok(p_url) => {
            //Check that the repo is a url
            if p_url.scheme() != "https" {
                return Err(format!("Failed to fetch the repository: Repository not a https url"));
            }

            //Extract repository name and owner
            let path_segments: Vec<&str> = p_url.path().split("/").collect();
            if path_segments.len() <= 2 {
                return Err(format!("Failed to parse repository owner and name from `{}`", p_url).to_owned());
            }

            repo_owner = path_segments[1].to_string();
            repo_name = path_segments[2].to_string();
            let host_str = p_url.host_str();
            match host_str {
                Some(h) => host_name = format!("{}-", h),
                None => host_name = "".to_string(),
            }

            host_name = host_name.replace(".", "-").to_string();
        }
    }

    //Clone the repository if it doesn't exist on disk
    let dest_path = format!(
        "{}/{}{}-{}",
        tmp_folder, host_name, repo_owner, repo_name
    );
    let git_repo: Repository;
    let path = path::Path::new(&dest_path);
    if !path.exists() {
        git_repo = match Repository::clone(url.as_str(), &dest_path) {
            Ok(git_repo) => git_repo,
            Err(err) => {
                return Err(format!("Failed to clone `{}` repository: {}", url, err).to_owned());
            }
        };
    } else {
        git_repo = match Repository::open(dest_path) {
            Ok(git_repo) => git_repo,
            Err(err) => {
                return Err(format!("Failed to clone `{}` repository: {}", url, err).to_owned());
            }
        };
    }

    let repo = Repo{repo: git_repo};
    Ok(repo)
}

pub struct Repo {
    repo: Repository,
}
