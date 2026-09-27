//! Binary entry — parse CLI and dispatch launch mode.
//!
//! WS-1: clap handles `--help` / `--version` (exit 0). Mode dispatch is WS-2.

use clap::Parser;
use mcload::cli::Args;

fn main() {
    let _args = Args::parse();
    // Mode dispatch (croft / loft / tray) lands in WS-2.
}
