//! WS-6 — Tray stub entrypoint (callable; native deps may be feature-gated later).

use mcload::tray;

#[test]
fn tray_dry_run_returns_ok() {
    tray::run(true).expect("tray dry-run stub should Ok(())");
}
