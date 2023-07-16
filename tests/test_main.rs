use assert_cmd::Command;
use std::net::TcpListener;

fn _get_available_port() -> String {
    format!(
        "{:?}",
        (9000..10000)
            .find(|port| _port_is_available(*port))
            .unwrap()
    )
}

fn _port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

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
        .arg(_get_available_port())
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .interrupted();
}

#[test]
fn dry_run() {
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--dry-run")
        .arg("--http-port")
        .arg(_get_available_port())
        .timeout(std::time::Duration::from_secs(1))
        .assert()
        .interrupted();
}
