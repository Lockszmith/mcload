//! CLI mode dispatch: `croft` / `loft` (+ dry-run seam). No top-level `tray`.

use assert_cmd::Command;
use clap::Parser;
use mcload::cli::{self, Args, LaunchMode};
use mcload::ui::loft::{self, LoftOptions};

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
fn help_lists_loft_no_tray_and_verbose() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    let assert = cmd.args(["loft", "--help"]).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let lower = stdout.to_ascii_lowercase();
    assert!(
        lower.contains("no-tray"),
        "loft help should list --no-tray:\n{stdout}"
    );
    assert!(
        lower.contains("verbose"),
        "loft help should list --verbose:\n{stdout}"
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
fn parse_loft_no_tray_and_verbose() {
    let loft = cli::parse_from(["mcload", "loft", "--no-tray", "--verbose"]);
    assert_eq!(
        cli::loft_options(&loft),
        LoftOptions {
            no_tray: true,
            verbose: true,
        }
    );
}

#[test]
fn parse_rejects_tray_subcommand() {
    let result = Args::try_parse_from(["mcload", "tray"]);
    assert!(result.is_err(), "top-level tray must not be a CLI mode");
}

#[test]
fn dispatch_dry_run_succeeds_for_croft_and_loft() {
    for mode in [LaunchMode::Croft, LaunchMode::Loft] {
        cli::dispatch(mode, true, LoftOptions::default()).unwrap_or_else(|e| {
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
fn tray_policy_respects_no_tray_flag() {
    // On Linux CI, tray_supported is false; should_use_tray must still be false with --no-tray
    // on every OS.
    assert!(!loft::should_use_tray(LoftOptions {
        no_tray: true,
        verbose: false,
    }));
    // Default: tray only when OS supports it.
    assert_eq!(
        loft::should_use_tray(LoftOptions::default()),
        loft::tray_supported()
    );
}
