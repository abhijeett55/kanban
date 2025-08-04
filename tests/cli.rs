use assert_cmd::prelude::*;

use std::process::Command;
#[test]
fn test_default_configuration_write() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kanban")?;
    cmd.arg("configure");
    cmd.assert()

        .stdout(predicates::str::contains("default configuration file"));
    Ok(())
}

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kanban")?;
    cmd.arg("help");
    cmd.assert()

        .stdout(predicates::str::contains("A command-line kanban board"));
    Ok(())
}