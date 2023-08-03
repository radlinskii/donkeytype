use std::io::Write;

use assert_cmd::Command;
use predicates;

#[test]
fn should_print_default_expected_input() {
    let mut temp_dict_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
    temp_dict_file
        .write_all(r#"hello world"#.as_bytes())
        .expect("Unable to write to temp file");

    let mut cmd = Command::cargo_bin("donkeytype").expect("Unable to get cargo bin");
    let cmd = cmd.arg(format!(
        "--dictionary-path={}",
        temp_dict_file.path().display()
    ));

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("hello"));
}

#[test]
fn should_print_help_message_for_normal_mode() {
    let mut cmd = Command::cargo_bin("donkeytype").expect("Unable to get cargo bin");

    cmd.assert().success().stdout(predicates::str::contains(
        "press 'e' to start editing, press 'q' to quit",
    ));
}
