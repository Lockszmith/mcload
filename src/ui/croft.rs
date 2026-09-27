//! Croft launch hook — FrankenTUI TTY (stub by default; `frankentui` feature later).

use crate::error::{Error, Result};

/// Run Croft UI. With `dry_run`, must return Ok without opening a TTY.
///
/// Stub stays RED until WS-5.
pub fn run(dry_run: bool) -> Result<()> {
    let _ = dry_run;
    Err(Error::NotImplemented("ui::croft::run — WS-5"))
}
