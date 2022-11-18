use crate::repo::Repo;
use std::path::Path;

pub struct Code {
    string: String
}

pub fn get_repo_path(repo: &Repo) -> Result<&Path, String> {
    match repo.repo.path().parent() {
        Some(repo_path) => return Ok(repo_path),
        None => {
            return Err(format!("Error"));
        }
    }
}

pub fn new(repo: &Repo) -> Result<Code, String> {
    let repo_path = get_repo_path(repo)?;
    println!("{:?}", repo_path.to_str());
    Ok(Code {string: "TMP".to_string()})
}
