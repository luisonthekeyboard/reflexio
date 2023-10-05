use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello").assert().success();
}


#[test]
fn compare_with_gnu_echo() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello").assert().success();
}