use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};

/// hash generates a sha256 hash of the `data`
pub fn new(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_bytes = hasher.finalize();
    HEXLOWER.encode(&hash_bytes)
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn generate_hash_from_data() {
        let data = "je suis con".to_string();
        assert_eq!(
            hash::new(data),
            "0603b541dc28322085f37ab24450a28097d54818fb86582a7848e8213739759d"
        );
    }
}
