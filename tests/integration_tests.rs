mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn cli_parse_outputs_json() {
    let temp = common::write_temp_file("key = value\n");

    let mut cmd = cargo_bin_cmd!("miniconf-parser");
    cmd.args(["parse", temp.path().to_str().unwrap(), "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"key\""));
}

#[test]
fn cli_check_reports_success() {
    let temp = common::write_temp_file(common::sample_document());

    let mut cmd = cargo_bin_cmd!("miniconf-parser");
    cmd.args(["check", temp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}
