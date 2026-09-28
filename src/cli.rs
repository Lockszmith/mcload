//! CLI args / subcommands / overrides.

use clap::{Parser, Subcommand};

use crate::ui::loft::LoftOptions;

/// Top-level CLI for the `mcload` binary.
#[derive(Debug, Clone, Parser, PartialEq, Eq)]
#[command(name = "mcload", version, about = "McLoad — There can be only one!")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to user/project config YAML.
    #[arg(long, global = true)]
    pub config: Option<std::path::PathBuf>,

    /// Project directory for per-project config.
    #[arg(long, global = true)]
    pub project: Option<std::path::PathBuf>,

    /// Override log level from CLI; wins over YAML.
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
    /// Loft — FrankenTUI Web UI (tray+BG on tray-capable OS)
    Loft {
        #[arg(long, default_value_t = false)]
        dry_run: bool,
        /// Force foreground web server (WSL / headless CI / no-tray OS).
        #[arg(long, default_value_t = false)]
        no_tray: bool,
        /// Allow CLI stdout/stderr (default tray loft is quiet aside from terminal URL).
        #[arg(long, default_value_t = false)]
        verbose: bool,
        /// Do not open the default browser on launch.
        #[arg(long, default_value_t = false)]
        no_browser: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode {
    Croft,
    Loft,
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
pub fn select_mode(args: &Args) -> Option<LaunchMode> {
    match args.command {
        Some(Command::Croft { .. }) => Some(LaunchMode::Croft),
        Some(Command::Loft { .. }) => Some(LaunchMode::Loft),
        None => None,
    }
}

/// Whether the selected mode was invoked with `--dry-run`.
pub fn dry_run(args: &Args) -> bool {
    match args.command {
        Some(Command::Croft { dry_run, .. }) | Some(Command::Loft { dry_run, .. }) => dry_run,
        None => false,
    }
}

/// Loft-only flags (`--no-tray` / `--verbose` / `--no-browser`); default when not loft.
pub fn loft_options(args: &Args) -> LoftOptions {
    match args.command {
        Some(Command::Loft {
            no_tray,
            verbose,
            no_browser,
            ..
        }) => LoftOptions {
            no_tray,
            verbose,
            no_browser,
        },
        _ => LoftOptions::default(),
    }
}

/// Dispatch a launch mode.
///
/// Dry-run returns `Ok` without opening UI (test seam only — not merge acceptance).
/// Non-dry-run forwards to Croft / Loft entrypoints.
pub fn dispatch(mode: LaunchMode, dry_run: bool, loft_opts: LoftOptions) -> crate::error::Result<()> {
    if dry_run {
        return Ok(());
    }
    match mode {
        LaunchMode::Croft => crate::ui::croft::run(false),
        LaunchMode::Loft => crate::ui::loft::run(false, loft_opts),
    }
}
