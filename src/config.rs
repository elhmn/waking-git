#[derive(Debug)]
pub struct Config {
    pub tmp_folder: String,
}

const DEFAULT_TMP_REPOSITORY: &str = "./tmp";

impl Config {
    pub fn new() -> Config {
        Config {
            tmp_folder: DEFAULT_TMP_REPOSITORY.to_string(),
        }
    }
}
