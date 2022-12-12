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
        let dir: PathBuf;

        // check if we are running the binary for integration tests
        if std::env::var("WAKE_TEST_MODE").is_ok() {
            dir = PathBuf::from("./");
        } else {
            dir = match home::home_dir() {
                Some(d) => d,
                None => {
                    return Config {
                        ..Default::default()
                    }
                }
            };
        }

        let wake_path = format!(
            "{}/{}",
            dir.to_str().unwrap_or(""),
            WAKE_FOLDER.to_string()
        );
        Config { wake_path }
    }
}
