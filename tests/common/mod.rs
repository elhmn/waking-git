use std::fs;
use std::path;

pub const TMP_DIR: &str = "/tmp/wake-tmp-folder";

pub fn move_to_tmp_folder() {
    //Create a temporary folder
    //Create the temporary directory if it doesn't exist
    let path = path::Path::new(TMP_DIR);
    if !path.exists() {
        fs::create_dir(TMP_DIR).unwrap();
    }
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
}
