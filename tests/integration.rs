use assert_cmd::Command;

#[test]
/// execute the help option
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("catalogue")?;
    let assert = cmd.arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
/// make sure the path exists
fn test_path() -> Result<()> {
    todo!();
    Ok(())
}