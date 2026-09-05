use assert_cmd::Command;

#[test]
#[ignore = "requires running Kubernetes cluster"]
fn runs() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(3))
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
fn invalid_mode() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--mode").arg("unsupported-mode").assert().failure();
}

#[test]
#[ignore = "requires running Kubernetes cluster"]
fn http_port() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(3))
        .assert()
        .interrupted();
}

#[test]
#[ignore = "requires running Kubernetes cluster"]
fn dry_run() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--dry-run")
        .arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(3))
        .assert()
        .interrupted();
}

#[test]
#[ignore = "requires running Kubernetes cluster"]
fn requeue_duration() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--requeue-duration")
        .arg("10")
        .arg("--http-port")
        .arg("0")
        .timeout(std::time::Duration::from_secs(3))
        .assert()
        .interrupted();
}
