//! WS-1 — Binary/crate smoke: `--help` / `--version` exit 0.
//!
//! Plan: `tests/cli_starts.rs` — binary prints version from `CARGO_PKG_VERSION`;
//! help lists modes once WS-2 lands (mode listing asserted in `cli_modes.rs`).

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn version_exits_zero_and_prints_pkg_version() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_exits_zero() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("mcload"));
}
