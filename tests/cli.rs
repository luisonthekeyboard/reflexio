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
fn compare_with_gnu_echo() {

    // generate test case
    let mut cmd = Command::new("echo");

    let skip_newline = thread_rng().gen_bool(0.5);

    if skip_newline {
        cmd.arg("-n");
    }

    let arg = random_string();
    cmd.arg(&arg);

    assert!(cmd.output().is_ok());
    let output = cmd.output().unwrap();
    let string_out = String::from_utf8(output.stdout).unwrap();

    let expected_output = if skip_newline { format!("{}", &arg) } else { format!("{}{}", &arg, "\n") };

    assert_eq!(&expected_output, &string_out);

    /////////////////////////////////////////////////

    let mut assert_cmd = Assert_Command::cargo_bin("echo").unwrap();
    if skip_newline {
        assert_cmd.arg("-n");
    }
    assert_cmd.arg(&arg);

    assert!(assert_cmd.output().is_ok());
    let assert_output = assert_cmd.output().unwrap();
    let assert_stdout = String::from_utf8(assert_output.stdout).unwrap();

    assert_eq!(&assert_stdout, &string_out);

}

fn random_string() -> String {
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(thread_rng().gen_range(1..17))
        .map(char::from)
        .collect();

    s
}