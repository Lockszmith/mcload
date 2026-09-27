//! Loft launch hook — FrankenTUI web/WASM host stub.

use crate::error::{Error, Result};

/// Run Loft UI. With `dry_run`, must return Ok without hosting WASM.
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    Err(Error::NotImplemented("ui::loft::run — FrankenTUI not wired"))
}
