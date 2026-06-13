//! Integration tests for the `bashlings` command layer.
//!
//! These run the built binary against the real repository workspace (the tests'
//! working directory is `cli/`, and `find_workspace_root` walks up to the repo
//! root). Assertions are chosen to be independent of which exercises are solved,
//! so they stay green regardless of local progress. Colors are disabled via
//! `NO_COLOR` so output matching is stable.

use assert_cmd::Command;
use predicates::str::contains;

fn bashlings() -> Command {
    let mut cmd = Command::cargo_bin("bashlings").unwrap();
    cmd.env("NO_COLOR", "1");
    cmd
}

#[test]
fn help_lists_all_commands() {
    bashlings()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("verify"))
        .stdout(contains("completions"))
        .stdout(contains("hint"));
}

#[test]
fn version_succeeds() {
    bashlings()
        .arg("--version")
        .assert()
        .success()
        .stdout(contains("bashlings"));
}

#[test]
fn list_shows_known_exercise() {
    bashlings()
        .arg("list")
        .assert()
        .success()
        .stdout(contains("intro1"));
}

#[test]
fn list_json_is_valid_and_has_total() {
    let out = bashlings().args(["list", "--json"]).assert().success();
    let stdout = String::from_utf8_lossy(&out.get_output().stdout).into_owned();
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(v["total"].as_u64(), Some(101));
    assert!(v["exercises"].is_array());
}

#[test]
fn no_color_strips_ansi() {
    let out = bashlings().arg("list").assert().success();
    let stdout = String::from_utf8_lossy(&out.get_output().stdout).into_owned();
    assert!(
        !stdout.contains('\x1b'),
        "NO_COLOR chiqishida ANSI bo'lmasligi kerak"
    );
}

#[test]
fn run_unknown_exercise_errors() {
    // dispatch returns Err -> exit code 2, message on stderr.
    bashlings()
        .args(["run", "nonexistent-xyz"])
        .assert()
        .code(2)
        .stderr(contains("topilmadi"));
}

#[test]
fn next_prints_a_pending_name() {
    // The repo always ships with pending exercises, so this succeeds.
    bashlings().arg("next").assert().success();
}

#[test]
fn completions_bash_generates_script() {
    bashlings()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(contains("bashlings"));
}
