pub mod code;
pub mod git;
use crate::config;
use crate::repo;
use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Data {
    pub git: git::Git,
    pub code: code::Code,
}

pub fn run(repo: &repo::Repo) -> Result<Data, String> {
    Ok(Data {
        git: git::new(repo)?,
        code: code::new(repo)?,
    })
}

pub fn extract(git_repo: &mut repo::Repo) -> Result<(Data, String), String> {
    let data = run(git_repo)?;
    let dest_path = format!("{}/{}", git_repo.scanner_path, config::EXTRACTOR_FILE_NAME);
    git_repo.extracted_file_path = dest_path.clone();
    let json_data = serde_json::to_string(&data).unwrap_or_else(|_| "".to_string());
    match utils::store_json_data(
        json_data.to_owned(),
        git_repo.scanner_path.to_owned(),
        &dest_path,
    ) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error: failed to extract repository data: {err}"));
        }
    };

    Ok((data, json_data))
}
