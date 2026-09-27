//! Binary entry — parse CLI and dispatch launch mode.
//!
//! WS-1: clap handles `--help` / `--version` (exit 0).
//! WS-2: mode dispatch (`croft` / `loft` / `tray`) with `--dry-run` seam.

use clap::Parser;
use mcload::cli::{self, Args};

fn main() {
    let args = Args::parse();
    if let Some(mode) = cli::select_mode(&args) {
        if let Err(err) = cli::dispatch(mode, cli::dry_run(&args)) {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
