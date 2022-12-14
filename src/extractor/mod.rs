pub mod code;
pub mod git;
use serde::Serialize;

use crate::repo;

#[derive(Serialize, Default)]
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
