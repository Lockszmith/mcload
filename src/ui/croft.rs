//! Croft launch hook — FrankenTUI TTY.

use std::io::IsTerminal;

use crate::error::{Error, Result};
use crate::ui::model::HelloTick;
use crate::ui::StartupReport;

const BACKEND_ID: &str = "frankentui-tty";

/// Non-blocking startup probe for tests / `MCLOAD_STARTUP_PROBE=1`.
///
/// Validates FrankenTUI TTY backend availability without requiring an
/// interactive session (CI stdout is often not a TTY). Does not enter the
/// event loop.
pub fn probe_startup() -> Result<StartupReport> {
    let compiled = ftui::DEFAULT_BACKEND;
    if compiled == "none" {
        return Err(Error::Ui(
            "FrankenTUI compiled without a TTY backend (enable ftui native-backend or crossterm)"
                .to_string(),
        ));
    }

    // Unix: headless ftui-tty backend (no raw mode). Windows uses crossterm via ftui.
    #[cfg(unix)]
    {
        let backend = ftui_tty::TtyBackend::new(80, 24);
        if backend.is_live() {
            return Err(Error::Ui(
                "expected headless TtyBackend::new to be non-live".to_string(),
            ));
        }
    }

    // Prove Model + App builder wire up against the runtime (no `.run()`).
    let _app = HelloTick::croft_app();

    Ok(StartupReport {
        backend: BACKEND_ID.to_string(),
        ready: true,
        detail: Some(format!(
            "ftui DEFAULT_BACKEND={compiled}; headless Croft probe ok"
        )),
    })
}

/// Run Croft UI. With `dry_run`, return Ok without opening a TTY (test seam).
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }

    if !std::io::stdout().is_terminal() {
        return Err(Error::TtyUnavailable(
            "Croft requires an interactive terminal (TTY). \
             Re-run in a real terminal, or use `--dry-run` / MCLOAD_STARTUP_PROBE=1."
                .to_string(),
        ));
    }

    HelloTick::croft_app()
        .run()
        .map_err(|e| Error::Ui(format!("FrankenTUI Croft run failed: {e}")))?;
    Ok(())
}
