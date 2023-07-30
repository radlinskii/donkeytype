use assert_cmd::Command;
use predicates;

#[test]
fn should_print_default_expected_input() {
    let mut cmd = Command::cargo_bin("donkeytype").unwrap();

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("hello world"));
}

#[test]
fn should_print_help_message_for_normal_mode() {
    let mut cmd = Command::cargo_bin("donkeytype").unwrap();

    cmd.write_stdin("e\n")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "press 'e' to start editing, press 'q' to quit",
        ));
}
