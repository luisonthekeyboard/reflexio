use std::process::Command;

use assert_cmd::Command as Assert_Command;
use predicates::prelude::predicate;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[test]
fn dies_no_args() {
    let mut cmd = Assert_Command::cargo_bin("echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Assert_Command::cargo_bin("echo").unwrap();
    cmd.arg("hello").assert().success();
}


#[test]
fn compare_with_system_echo() {

    // generate test case
    let skip_newline = thread_rng().gen_bool(0.5);
    let arg = random_string();

    // run the system version
    let mut system_cmd = Command::new("echo");
    if skip_newline {
        system_cmd.arg("-n");
    }
    system_cmd.arg(&arg);

    assert!(system_cmd.output().is_ok(), "Calling system command failed.");

    let system_cmd_output = String::from_utf8(system_cmd.output().unwrap().stdout).unwrap();
    let expected_output = if skip_newline { format!("{}",&arg) } else { format!("{}{}", &arg, "\n") };

    assert_eq!(expected_output, system_cmd_output, "Test case failed for system command");

    // run the local version under test

    let mut cmd_under_test = Assert_Command::cargo_bin("echo").unwrap();
    if skip_newline {
        cmd_under_test.arg("-n");
    }
    cmd_under_test.arg(&arg);

    assert!(cmd_under_test.output().is_ok(), "Calling command under test failed");

    let assert_output = cmd_under_test.output().unwrap();
    let assert_stdout = String::from_utf8(assert_output.stdout).unwrap();

    assert_eq!(&assert_stdout, &system_cmd_output, "Command under test didn't produce the same output as the system command");
}

fn random_string() -> String {
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(thread_rng().gen_range(1..17))
        .map(char::from)
        .collect();

    s
}