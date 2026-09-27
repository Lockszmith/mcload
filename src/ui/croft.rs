//! Croft launch hook — FrankenTUI TTY.

use crate::error::{Error, Result};
use crate::ui::StartupReport;

/// Non-blocking startup probe for tests / `MCLOAD_STARTUP_PROBE=1`.
///
/// Must initialize the FrankenTUI TTY backend (or return a clear actionable
/// error only when the environment truly cannot run a TTY UI).
pub fn probe_startup() -> Result<StartupReport> {
    Err(Error::NotImplemented(
        "ui::croft::probe_startup — FrankenTUI TTY not wired (R-WS-3)",
    ))
}

/// Run Croft UI. With `dry_run`, return Ok without opening a TTY (test seam).
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    let _report = probe_startup()?;
    Err(Error::NotImplemented(
        "ui::croft::run — FrankenTUI interactive loop not wired (R-WS-3)",
    ))
}
