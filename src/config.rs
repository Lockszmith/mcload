//! YAML config load/save with CLI overrides (WS-3).
//!
//! Merge order (contract): defaults < user YAML < project YAML < CLI.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiConfig {
    pub default_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub data_dir: PathBuf,
    pub log_level: String,
    pub ui: UiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("."),
            log_level: "info".to_string(),
            ui: UiConfig {
                default_mode: "croft".to_string(),
            },
        }
    }
}

/// Defaults when no config file exists.
pub fn defaults() -> Config {
    Config::default()
}

/// Load config from a YAML file. Missing file → defaults (contract).
///
/// Stub always returns [`Error::NotImplemented`] so WS-3 tests fail.
pub fn load_file(_path: &Path) -> Result<Config> {
    Err(Error::NotImplemented("config::load_file — WS-3"))
}

/// Persist config as YAML.
pub fn save_file(_path: &Path, _cfg: &Config) -> Result<()> {
    Err(Error::NotImplemented("config::save_file — WS-3"))
}

/// Apply CLI overrides on top of a base config. CLI wins.
///
/// Stub returns `base` unchanged so override tests fail.
pub fn apply_cli_overrides(base: Config, _log_level: Option<&str>) -> Config {
    base
}
