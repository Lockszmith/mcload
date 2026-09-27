//! CLI args / subcommands / overrides (WS-2 owns modes; WS-3 owns config flags).

use clap::{Parser, Subcommand};

/// Top-level CLI for the `mcload` binary.
#[derive(Debug, Clone, Parser, PartialEq, Eq)]
#[command(name = "mcload", version, about = "McLoad — There can be only one!")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to user/project config YAML (WS-3).
    #[arg(long, global = true)]
    pub config: Option<std::path::PathBuf>,

    /// Project directory for per-project config (WS-3).
    #[arg(long, global = true)]
    pub project: Option<std::path::PathBuf>,

    /// Override log level from CLI (WS-3); wins over YAML.
    #[arg(long, global = true)]
    pub log_level: Option<String>,
}

#[derive(Debug, Clone, Subcommand, PartialEq, Eq)]
pub enum Command {
    /// Croft — FrankenTUI TTY UI
    Croft {
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Loft — FrankenTUI Web/WASM UI
    Loft {
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
    /// Tray / background mode
    Tray {
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode {
    Croft,
    Loft,
    Tray,
}

/// Parse argv-style args (without binary name).
pub fn parse_from<I, T>(args: I) -> Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    Args::parse_from(args)
}

/// Select launch mode from parsed args.
///
/// Stub returns `None` always so WS-2 dispatch tests fail until implemented.
pub fn select_mode(args: &Args) -> Option<LaunchMode> {
    let _ = args;
    None
}

/// Dispatch a launch mode (stub / dry-run seam).
///
/// Stub always errors so mode-handler tests stay RED until WS-2.
pub fn dispatch(mode: LaunchMode, dry_run: bool) -> crate::error::Result<()> {
    let _ = (mode, dry_run);
    Err(crate::error::Error::NotImplemented(
        "cli::dispatch — WS-2 mode stubs",
    ))
}
