use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Config {
    /// Path to the wake folder
    /// here we store everything related to wake
    pub wake_path: String,
}

const WAKE_FOLDER: &str = ".wake";

impl Config {
    pub fn new() -> Config {
        

        // check if we are running the binary for integration tests
        let dir: PathBuf = if std::env::var("WAKE_TEST_MODE").is_ok() {
            PathBuf::from("./")
        } else {
            match home::home_dir() {
                Some(d) => d,
                None => {
                    return Config {
                        ..Default::default()
                    }
                }
            }
        };

        let wake_path = format!("{}/{}", dir.to_str().unwrap_or(""), WAKE_FOLDER);
        Config { wake_path }
    }
}
