use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn arg_only_10() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rtimer")?;
    cmd.arg("10");
    cmd.assert().success();
    Ok(())
}

#[test]
fn arg_5_sec() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rtimer")?;
    cmd.arg("5").arg("sec");
    cmd.assert().success();
    Ok(())
}

#[test]
fn arg_5_sec_with_progress() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rtimer")?;
    cmd.arg("5").arg("sec").arg("-p").arg("01");
    cmd.assert().success();
    Ok(())
}
