//! Binary entry — parse CLI and dispatch launch mode.
//!
//! Modes: `croft` (TTY) / `loft` (Web). `--dry-run` is a test seam only.

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
