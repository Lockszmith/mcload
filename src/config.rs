//! YAML config load/save with CLI overrides (WS-3).
//!
//! Merge order (contract): defaults < user YAML < project YAML < CLI.

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
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

/// Resolve the default user config directory (XDG / AppData via `directories`).
pub fn user_config_dir() -> Result<PathBuf> {
    ProjectDirs::from("com", "mcload", "mcload")
        .map(|d| d.config_dir().to_path_buf())
        .ok_or_else(|| Error::Config("could not resolve user config directory".into()))
}

/// Default user config file path (`config.yaml` under [`user_config_dir`]).
pub fn user_config_path() -> Result<PathBuf> {
    Ok(user_config_dir()?.join("config.yaml"))
}

/// Load config from a YAML file. Missing file → defaults (contract).
pub fn load_file(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(defaults());
    }
    let raw = fs::read_to_string(path)?;
    let cfg: Config = serde_yaml::from_str(&raw)?;
    Ok(cfg)
}

/// Persist config as YAML.
pub fn save_file(path: &Path, cfg: &Config) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let raw = serde_yaml::to_string(cfg)?;
    fs::write(path, raw)?;
    Ok(())
}

/// Apply CLI overrides on top of a base config. CLI wins.
pub fn apply_cli_overrides(mut base: Config, log_level: Option<&str>) -> Config {
    if let Some(level) = log_level {
        base.log_level = level.to_string();
    }
    base
}
