//! CLI mode dispatch: `croft` / `loft` (+ dry-run seam). No top-level `tray`.

use assert_cmd::Command;
use clap::Parser;
use mcload::cli::{self, Args, LaunchMode};

#[test]
fn help_lists_croft_loft_not_tray() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    let assert = cmd.arg("--help").assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.to_ascii_lowercase().contains("croft"),
        "help should list croft"
    );
    assert!(
        stdout.to_ascii_lowercase().contains("loft"),
        "help should list loft"
    );
    // Commands section must not advertise a top-level `tray` mode.
    let has_tray_command = stdout.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("tray ") || trimmed == "tray"
    });
    assert!(
        !has_tray_command,
        "help must not list tray as a top-level command:\n{stdout}"
    );
}

#[test]
fn parse_selects_croft_and_loft() {
    let croft = cli::parse_from(["mcload", "croft", "--dry-run"]);
    assert_eq!(cli::select_mode(&croft), Some(LaunchMode::Croft));

    let loft = cli::parse_from(["mcload", "loft", "--dry-run"]);
    assert_eq!(cli::select_mode(&loft), Some(LaunchMode::Loft));
}

#[test]
fn parse_rejects_tray_subcommand() {
    let result = Args::try_parse_from(["mcload", "tray"]);
    assert!(result.is_err(), "top-level tray must not be a CLI mode");
}

#[test]
fn dispatch_dry_run_succeeds_for_croft_and_loft() {
    for mode in [LaunchMode::Croft, LaunchMode::Loft] {
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
