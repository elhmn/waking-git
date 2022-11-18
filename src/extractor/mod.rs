pub mod git;

use crate::repo;

pub struct Data {
    pub git: git::Git,
}

pub fn run(repo: repo::Repo) -> Data {
    println!("main extractor called!");
    return Data {
        git: git::new(repo),
    }
}
