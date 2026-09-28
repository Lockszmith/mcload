//! Binary entry — parse CLI and dispatch launch mode.
//!
//! Modes: `croft` (TTY) / `loft` (Web). `--dry-run` is a test seam only.
//! `MCLOAD_STARTUP_PROBE=1` runs a non-blocking FrankenTUI startup probe (CI seam).
//! Loft: `--no-tray` / `--verbose`; tray is default-on when OS supports it (not a CLI mode).

use clap::Parser;
use mcload::cli::{self, Args, LaunchMode};

fn main() {
    let args = Args::parse();
    if let Some(mode) = cli::select_mode(&args) {
        // Non-blocking CI / contract seam — must not open an interactive UI loop.
        if std::env::var_os("MCLOAD_STARTUP_PROBE").is_some_and(|v| v == "1") {
            let result = match mode {
                LaunchMode::Croft => mcload::ui::croft::probe_startup(),
                LaunchMode::Loft => mcload::ui::loft::probe_startup(),
            };
            match result {
                Ok(report) => {
                    println!("{} ready={}", report.backend, report.ready);
                    if let Some(detail) = &report.detail {
                        println!("{detail}");
                    }
                    return;
                }
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1);
                }
            }
        }

        if let Err(err) = cli::dispatch(mode, cli::dry_run(&args), cli::loft_options(&args)) {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
