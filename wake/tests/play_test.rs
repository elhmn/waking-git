use assert_cmd::prelude::*;
use core::utils::test;
// Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn should_work_correctly() -> Result<(), Box<dyn std::error::Error>> {
    test::setup();

    let url = "https://github.com/elhmn/ckp";

    let mut cmd = Command::cargo_bin("wake")?;
    cmd.arg("play").arg("shmup").arg(url);

    //Test that the player is running
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running the player..."));

    test::teardown();
    Ok(())
}
