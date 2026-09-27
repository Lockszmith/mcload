//! System tray stub (WS-6). Behind `feature = "tray"` for native deps later;
//! default build still exposes a callable stub entry for tests.

use crate::error::{Error, Result};

/// Run tray / background idle loop stub.
///
/// With `dry_run`, must return Ok quickly without creating a native tray icon.
pub fn run(dry_run: bool) -> Result<()> {
    if dry_run {
        return Ok(());
    }
    Err(Error::NotImplemented("tray::run — WS-6"))
}
