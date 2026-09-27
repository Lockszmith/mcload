//! UI launch surfaces — Croft (TTY) and Loft (Web).

pub mod croft;
pub mod loft;
pub(crate) mod model;

/// Report from a non-blocking FrankenTUI startup probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupReport {
    /// Backend id, e.g. `frankentui-tty` or `frankentui-web`.
    pub backend: String,
    /// True when the UI backend initialized successfully.
    pub ready: bool,
    /// Optional detail (listen URL, TTY note, actionable error context).
    pub detail: Option<String>,
}
