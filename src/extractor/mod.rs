pub mod code;
pub mod git;

use crate::repo;

#[derive(Default)]
pub struct Data {
    pub git: git::Git,
    pub code: code::Code,
}

pub fn run(repo: repo::Repo) -> Result<Data, String> {
    return Ok(Data {
        git: git::new(&repo)?,
//         code: code::new(&repo)?,
        ..Default::default()
    })
}
