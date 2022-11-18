use assert_cmd::prelude::*;
use common::TMP_DIR;
// Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

mod common;

#[test]
fn fail_to_parse_wrong_url() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();

    struct Test<'a> {
        url: &'a str,
        exp: &'a str,
    }

    let tests = [
        Test {
            url: "./test/file/doesnt/exist",
            exp: "Failed to parse",
        },
        Test {
            url: "http://github.com",
            exp: "not a https url",
        },
        Test {
            url: "file://github.com",
            exp: "not a https url",
        },
        Test {
            url: "https://githubcom/elhmn/ckp",
            exp: "failed to resolve address for githubcom",
        },
        Test {
            url: "http://",
            exp: "Failed to parse",
        },
        Test {
            url: "",
            exp: "Failed to parse",
        },
    ];

    for t in tests {
        let mut cmd = Command::cargo_bin("wake")?;
        cmd.current_dir(common::TMP_DIR).arg("scan").arg(t.url);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(t.exp));
    }

    common::teardown();
    Ok(())
}

#[test]
fn clone_repository() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();
    let url = "https://github.com/elhmn/ckp";

    let mut cmd = Command::cargo_bin("wake")?;
    cmd.current_dir(TMP_DIR).arg("scan").arg(url);

    //we should be able clone the repository successfully
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("repository cloned successfully"));

    //the ./tmp/github-com-elhmn-ckp directory should be created
    let expected_dir = format!("{}/{}/{}", TMP_DIR, "tmp", "github-com-elhmn-ckp");
    assert!(std::path::Path::new(expected_dir.as_str()).exists());

    common::teardown();
    Ok(())
}

#[test]
fn doesnt_fetch_repository_if_already_exists() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();
    let url = "https://github.com/elhmn/ckp";

    let mut cmd = Command::cargo_bin("wake")?;
    cmd.current_dir(TMP_DIR).arg("scan").arg(url);

    //we should be able clone the repository successfully the first time
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("repository cloned successfully"));

    //then work even though the repository already exist on disk 
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("repository cloned successfully"));

    common::teardown();
    Ok(())
}