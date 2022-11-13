use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn fail_to_parse_wrong_url() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("scan").arg(t.url);
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(t.exp));
    }

    Ok(())
}
