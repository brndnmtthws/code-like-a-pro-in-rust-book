use assert_cmd::Command;

#[test]
fn test_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("quicksort-cli")?;
    cmd.assert().success().stdout("[]\n");

    Ok(())
}

#[test]
fn test_cli_well_known() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("quicksort-cli")?;
    cmd.args(&["14", "52", "1", "-195", "1582"])
        .assert()
        .success()
        .stdout("[-195, 1, 14, 52, 1582]\n");

    Ok(())
}

#[test]
fn test_cli_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    let paths = fs::read_dir("tests/fixtures")?;

    for fixture in paths {
        let mut path = fixture?.path();
        path.push("args");
        let args: Vec<String> = fs::read_to_string(&path)?
            .trim()
            .split(' ')
            .map(str::to_owned)
            .collect();
        path.pop();
        path.push("expected");
        let expected = fs::read_to_string(&path)?;

        let mut cmd = Command::cargo_bin("quicksort-cli")?;
        cmd.args(args).assert().success().stdout(expected);
    }

    Ok(())
}
