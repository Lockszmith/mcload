//! McLoad library surface for integration tests and shared modules.

pub mod cli;
pub mod config;
pub mod error;
pub mod metadata;
pub mod plugins;
pub mod queues;
pub mod ui;

pub use error::Error;
