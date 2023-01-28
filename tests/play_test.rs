use assert_cmd::prelude::*;
use common::TMP_DIR;
// Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

mod common;

#[test]
fn should_work_correctly() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    let url = "https://github.com/elhmn/ckp";

    let mut cmd = Command::cargo_bin("wake")?;
    cmd.current_dir(TMP_DIR).arg("play").arg(url);

    //Test that the player is running
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running the player..."));

    common::teardown();
    Ok(())
}
