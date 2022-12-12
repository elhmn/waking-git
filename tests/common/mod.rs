use std::fs;
use std::path;
use std::env;

pub const TMP_DIR: &str = "/tmp/wake-tmp-folder";

pub fn move_to_tmp_folder() {
    //Create a temporary folder
    //Create the temporary directory if it doesn't exist
    let path = path::Path::new(TMP_DIR);
    if !path.exists() {
        fs::create_dir(TMP_DIR).unwrap();
    }

    //Set test mode
    env::set_var("WAKE_TEST_MODE", "1");
}

pub fn delete_tmp_folder() {
    let _ = fs::remove_dir_all(TMP_DIR); //silence the error
}

pub fn setup() {
    delete_tmp_folder();
    move_to_tmp_folder();
}

pub fn teardown() {
    delete_tmp_folder();
    env::remove_var("WAKE_TEST_MODE");
}
