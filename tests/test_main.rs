use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.timeout(std::time::Duration::from_secs(1))
        .assert()
        .interrupted();
}

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn version() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn http_port() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .interrupted();
}

#[test]
fn dry_run() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--dry-run")
        .arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .interrupted();
}

#[test]
fn requeue_duration() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--requeue-duration")
        .arg("10")
        .arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .interrupted();
}
