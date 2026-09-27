//! McLoad library surface for integration tests and shared modules.
//!
//! Scaffolding stubs intentionally incomplete — tests assert contracts (RED until WS impl).

pub mod cli;
pub mod config;
pub mod error;
pub mod metadata;
pub mod plugins;
pub mod queues;
pub mod tray;
pub mod ui;

pub use error::Error;
