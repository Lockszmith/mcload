//! Loft launch hook — FrankenTUI web/WASM host stub.

use crate::error::{Error, Result};

/// Run Loft UI. With `dry_run`, must return Ok without hosting WASM.
///
/// Stub stays RED until WS-5.
pub fn run(dry_run: bool) -> Result<()> {
    let _ = dry_run;
    Err(Error::NotImplemented("ui::loft::run — WS-5"))
}
