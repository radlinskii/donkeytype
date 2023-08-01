use std::io::Write;

use assert_cmd::Command;
use predicates;

#[test]
fn should_print_default_expected_input() {
    let mut temp_dict_file = tempfile::NamedTempFile::new().unwrap();
    temp_dict_file
        .write_all(r#"hello world - some other words are here too"#.as_bytes())
        .unwrap();

    let mut cmd = Command::cargo_bin("donkeytype").unwrap();
    let cmd = cmd.arg(format!(
        "--dictionary-path={}",
        temp_dict_file.path().display()
    ));

    cmd.assert()
        .success()
        .stdout(predicates::str::contains("hello world"));
}

#[test]
fn should_print_help_message_for_normal_mode() {
    let mut cmd = Command::cargo_bin("donkeytype").unwrap();

    cmd.assert().success().stdout(predicates::str::contains(
        "press 'e' to start editing, press 'q' to quit",
    ));
}
