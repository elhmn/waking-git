use std::path::PathBuf;

pub const REPOS_FOLDER_NAME: &str = "repos";
pub const SCANNER_FOLDER_NAME: &str = "scanner";
pub const EXTRACTOR_FILE_NAME: &str = "extracted.json";
pub const CONVERTER_FILE_NAME_PREFIX: &str = "converted.json";

#[derive(Debug, Default)]
pub struct Config {
    /// Path to the wake folder
    /// here we store everything related to wake
    pub wake_path: String,

    /// Path to the repository storage folder
    pub storage_path: String,
}

pub const WAKE_FOLDER: &str = ".wake";

impl Config {
    pub fn new() -> Config {
        let dir = get_home_dir();
        let wake_path = format!("{dir}/{WAKE_FOLDER}");
        let storage_path = format!("{wake_path}/{REPOS_FOLDER_NAME}");
        Config {
            wake_path,
            storage_path,
        }
    }
}

pub fn get_home_dir() -> String {
    // check if we are running the binary for integration tests
    let dir: PathBuf = if std::env::var("WAKE_TEST_MODE").is_ok() {
        PathBuf::from("./")
    } else {
        match home::home_dir() {
            Some(d) => d,
            None => return "".to_string(),
        }
    };

    dir.to_str().unwrap_or("").to_string()
}
