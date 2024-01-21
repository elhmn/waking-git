use crate::config;
use std::fs;
use std::io;
use std::path;
use std::process::Command;

pub fn run_player(player: String, file: String) -> Result<String, io::Error> {
    let players_bin = get_players_bin();

    //check that players_bin file exists
    if !path::Path::new(&players_bin).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("`{players_bin}` file not found"),
        ));
    }

    let output = Command::new(players_bin)
        .arg(player)
        .arg(file)
        .spawn()?
        .wait_with_output()?;

    if !output.status.success() {
        let err = String::from_utf8(output.stderr).unwrap_or(String::from(""));
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }

    let out = String::from_utf8(output.stdout).unwrap_or(String::from(""));
    Ok(out)
}

fn get_players_bin() -> String {
    let dir = config::get_home_dir();
    let wake_path = format!("{}/{}", dir, config::WAKE_FOLDER);
    let bin = format!("{wake_path}/bin");

    //Create the bin folder if it doesn't exist
    let path = path::Path::new(&bin);
    if !path.exists() {
        match fs::create_dir_all(&bin) {
            Ok(_) => {}
            Err(err) => {
                println!("Error: failed to create players bin folder: {err}");
            }
        }
    }

    format!("{}/{}", bin, "players")
}
