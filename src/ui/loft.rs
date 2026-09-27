//! Loft launch hook — FrankenTUI Web UI (optional tray under Loft later).

use crate::error::{Error, Result};
use crate::ui::StartupReport;

/// Non-blocking startup probe for tests / `MCLOAD_STARTUP_PROBE=1`.
///
/// Must initialize the FrankenTUI Web UI path and report a listen URL/detail.
pub fn probe_startup() -> Result<StartupReport> {
    Err(Error::NotImplemented(
        "ui::loft::probe_startup — FrankenTUI Web not wired (R-WS-3)",
    ))
}

/// Run Loft UI. With `dry_run`, return Ok without hosting Web UI (test seam).
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    let _report = probe_startup()?;
    Err(Error::NotImplemented(
        "ui::loft::run — FrankenTUI Web loop not wired (R-WS-3)",
    ))
}
