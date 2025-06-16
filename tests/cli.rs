use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chicago_project_zero_fatalities_parser")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

// TODO: more testing.
// e.g. cannot write to file, cannot parse file, cannot create file, etc.
