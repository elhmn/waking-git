use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

mod common;

#[test]
fn should_work_correctly() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let mut cmd = Command::cargo_bin("wake")?;
    cmd.current_dir(common::TMP_DIR).arg("play");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Play run command invoked"));

    common::teardown();
    Ok(())
}

#[test]
fn should_work_correctly_with_dir_flag_set() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let mut cmd = Command::cargo_bin("wake")?;
    cmd.current_dir(common::TMP_DIR)
        .arg("play")
        .arg("-d a_directory");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Play run command invoked"));

    common::teardown();
    Ok(())
}
