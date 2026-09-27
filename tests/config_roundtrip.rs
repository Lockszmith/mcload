//! WS-3 — Config YAML: defaults when missing; load/save; CLI override wins.
//!
//! Wave C: runs AFTER WS-2 (cli flag wiring). Do not merge WS-2 ‖ WS-3 on `cli.rs`.

use mcload::config;
use std::fs;
use tempfile::tempdir;

#[test]
fn missing_file_loads_defaults() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("missing.yaml");
    assert!(!path.exists());

    let cfg = config::load_file(&path).expect("missing file → defaults");
    let defaults = config::defaults();
    assert_eq!(cfg.log_level, defaults.log_level);
    assert_eq!(cfg.ui.default_mode, defaults.ui.default_mode);
}

#[test]
fn save_and_reload_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("config.yaml");

    let mut cfg = config::defaults();
    cfg.log_level = "debug".to_string();
    cfg.ui.default_mode = "loft".to_string();

    config::save_file(&path, &cfg).expect("save");
    assert!(path.is_file());
    let raw = fs::read_to_string(&path).expect("read yaml");
    assert!(
        raw.contains("debug") || raw.contains("log_level"),
        "saved YAML should contain config fields"
    );

    let loaded = config::load_file(&path).expect("reload");
    assert_eq!(loaded, cfg);
}

#[test]
fn cli_override_wins_over_file() {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join("config.yaml");

    let mut cfg = config::defaults();
    cfg.log_level = "warn".to_string();
    config::save_file(&path, &cfg).expect("save");

    let loaded = config::load_file(&path).expect("load");
    let merged = config::apply_cli_overrides(loaded, Some("error"));
    assert_eq!(
        merged.log_level, "error",
        "CLI --log-level must win over YAML"
    );
}
