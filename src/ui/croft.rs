//! Croft launch hook — FrankenTUI TTY (stub by default; `frankentui` feature later).

use crate::error::{Error, Result};

/// Run Croft UI. With `dry_run`, must return Ok without opening a TTY.
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    Err(Error::NotImplemented("ui::croft::run — FrankenTUI not wired"))
}
