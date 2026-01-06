// use assert_cmd::cargo;
// use assert_cmd::prelude::*;
// use predicates::prelude::*;
// use std::process::Command;

// #[test]
// fn success() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::new(cargo::cargo_bin!("chicago_project_zero_fatalities_parser"));
//     cmd.assert()
//         .success()
//         .stdout(predicate::str::contains("Finished writing to file"));

//     Ok(())
// }

// TODO: more testing.
// e.g. cannot write to file, cannot parse file, cannot create file, etc.
