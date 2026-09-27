//! R-WS-4 — Real FrankenTUI startup contracts (intentionally RED until R-WS-3).
//!
//! Dry-run success alone is NOT acceptance. These probes must prove Croft TTY
//! and Loft Web backends initialize via FrankenTUI.

use assert_cmd::Command;
use mcload::ui::{croft, loft};

#[test]
fn croft_probe_startup_reports_frankentui_tty() {
    let report = croft::probe_startup().unwrap_or_else(|e| {
        panic!("croft::probe_startup must initialize FrankenTUI TTY (or actionable TTY error): {e}");
    });
    assert!(
        report.ready,
        "croft probe must report ready when TTY UI can start"
    );
    assert!(
        report.backend.contains("frankentui") && report.backend.contains("tty"),
        "expected frankentui tty backend, got {:?}",
        report.backend
    );
}

#[test]
fn loft_probe_startup_reports_frankentui_web() {
    let report = loft::probe_startup().unwrap_or_else(|e| {
        panic!("loft::probe_startup must initialize FrankenTUI Web UI: {e}");
    });
    assert!(
        report.ready,
        "loft probe must report ready when Web UI can start"
    );
    assert!(
        report.backend.contains("frankentui") && report.backend.contains("web"),
        "expected frankentui web backend, got {:?}",
        report.backend
    );
    assert!(
        report
            .detail
            .as_deref()
            .is_some_and(|d| d.contains("http://") || d.contains("listen")),
        "loft probe detail should include a listen URL or listen note, got {:?}",
        report.detail
    );
}

#[test]
fn croft_binary_startup_probe_succeeds() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    let assert = cmd
        .env("MCLOAD_STARTUP_PROBE", "1")
        .args(["croft"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.to_ascii_lowercase().contains("frankentui")
            && stdout.to_ascii_lowercase().contains("tty"),
        "croft probe stdout should mention frankentui tty backend:\n{stdout}"
    );
}

#[test]
fn loft_binary_startup_probe_succeeds() {
    let mut cmd = Command::cargo_bin("mcload").expect("mcload binary");
    let assert = cmd
        .env("MCLOAD_STARTUP_PROBE", "1")
        .args(["loft"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let lower = stdout.to_ascii_lowercase();
    assert!(
        lower.contains("frankentui") && lower.contains("web"),
        "loft probe stdout should mention frankentui web backend:\n{stdout}"
    );
}

#[test]
fn croft_non_dry_run_must_not_be_not_implemented_stub() {
    // Library path without dry-run must not stay on the scaffolding NotImplemented stub.
    // Probe is the non-blocking contract; full interactive run is for manual MC confirm.
    let err = match croft::probe_startup() {
        Ok(_) => return,
        Err(e) => e,
    };
    let msg = err.to_string().to_ascii_lowercase();
    assert!(
        !msg.contains("not implemented"),
        "croft must not remain a NotImplemented stub; got: {msg}"
    );
}

#[test]
fn loft_non_dry_run_must_not_be_not_implemented_stub() {
    let err = match loft::probe_startup() {
        Ok(_) => return,
        Err(e) => e,
    };
    let msg = err.to_string().to_ascii_lowercase();
    assert!(
        !msg.contains("not implemented"),
        "loft must not remain a NotImplemented stub; got: {msg}"
    );
}
