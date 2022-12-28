pub mod code;
pub mod git;
use data_encoding::HEXLOWER;
use serde::Serialize;
use sha2::{Digest, Sha256};

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

///hash generate a sha256 hash of the `data`
pub fn hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_bytes = hasher.finalize();
    HEXLOWER.encode(&hash_bytes)
}

#[cfg(test)]
mod tests {
    use crate::extractor::hash;

    #[test]
    fn generate_hash_from_data() {
        let data = "je suis con".to_string();
        assert_eq!(
            hash(data),
            "0603b541dc28322085f37ab24450a28097d54818fb86582a7848e8213739759d"
        );
    }
}
