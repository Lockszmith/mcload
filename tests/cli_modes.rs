//! WS-2 — CLI mode dispatch: `croft`, `loft`, `tray` (+ dry-run seam).
//!
//! Depends on WS-1 binary/lib surface. WS-3 config flags are NOT asserted here
//! (Wave C owns `--config` / `--project` / override wiring on `cli.rs`).

use assert_cmd::Command;
use mcload::cli::{self, LaunchMode};
use predicates::prelude::*;

#[test]
fn help_lists_croft_loft_tray_modes() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("croft"))
        .stdout(predicate::str::contains("loft"))
        .stdout(predicate::str::contains("tray"));
}

#[test]
fn parse_selects_croft_loft_tray() {
    let croft = cli::parse_from(["mcload", "croft", "--dry-run"]);
    assert_eq!(cli::select_mode(&croft), Some(LaunchMode::Croft));

    let loft = cli::parse_from(["mcload", "loft", "--dry-run"]);
    assert_eq!(cli::select_mode(&loft), Some(LaunchMode::Loft));

    let tray = cli::parse_from(["mcload", "tray", "--dry-run"]);
    assert_eq!(cli::select_mode(&tray), Some(LaunchMode::Tray));
}

#[test]
fn dispatch_dry_run_succeeds_for_each_mode() {
    for mode in [LaunchMode::Croft, LaunchMode::Loft, LaunchMode::Tray] {
        cli::dispatch(mode, true).unwrap_or_else(|e| {
            panic!("dispatch({mode:?}, dry_run=true) should Ok: {e}");
        });
    }
}

#[test]
fn croft_dry_run_binary_exits_zero() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    cmd.args(["croft", "--dry-run"]).assert().success();
}

#[test]
fn loft_dry_run_binary_exits_zero() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    cmd.args(["loft", "--dry-run"]).assert().success();
}

#[test]
fn tray_dry_run_binary_exits_zero() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    cmd.args(["tray", "--dry-run"]).assert().success();
}
