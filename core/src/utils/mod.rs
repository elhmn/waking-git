pub mod test;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path;

pub fn store_json_data(data: String, dest_folder: String, dest_path: &String) -> Result<(), Error> {
    let path = path::Path::new(&dest_folder);
    if !path.exists() {
        fs::create_dir_all(&dest_folder)?
    }

    let mut file = File::create(dest_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
