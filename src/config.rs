use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub tmp_folder: String,
}

const DEFAULT_TMP_REPOSITORY: &str = "tmp";

impl Config {
    pub fn new() -> Config {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tmp_folder = format!("{}/{}", dir.to_str().unwrap_or(""), DEFAULT_TMP_REPOSITORY.to_string());
        Config {
            tmp_folder,
        }
    }
}
