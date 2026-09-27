# 0001 — Scaffolding test RED log

Captured after scaffolding; failures are expected (NotImplemented stubs).

## Command

```
$mingwBin = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin"
$env:Path = "$mingwBin;$env:USERPROFILE\.cargo\bin;$env:Path"
cargo +stable-x86_64-pc-windows-gnu test --no-fail-fast
```

Note: plain `cargo test --no-fail-fast` failed — `cargo` not on PATH.

## Summary

| Metric  | Count |
| ------- | ----- |
| passed  | 0     |
| failed  | 25    |
| ignored | 0     |
| status  | RED   |

**Verdict:** RED (failures expected)

## Failures by target

| Target                       | Passed | Failed | Ignored |
| ---------------------------- | ------ | ------ | ------- |
| `--lib`                      | 0      | 3      | 0       |
| `--bin` (main)               | 0      | 0      | 0       |
| `--test cli_modes`           | 0      | 6      | 0       |
| `--test cli_starts`          | 0      | 2      | 0       |
| `--test config_roundtrip`    | 0      | 3      | 0       |
| `--test metadata_concurrent` | 0      | 2      | 0       |
| `--test queues_stubs`        | 0      | 3      | 0       |
| `--test tray_stub`           | 0      | 1      | 0       |
| `--test ui_plugin_stubs`     | 0      | 5      | 0       |
| doc-tests                    | 0      | 0      | 0       |

### `--lib`

| Test                                                               |
| ------------------------------------------------------------------ |
| `queues::activity::tests::activity_pause_resume_abort_transitions` |
| `queues::gathering::tests::gathering_marks_items_fresh`            |
| `queues::reckoning::tests::reckoning_promotes_fresh_to_ready`      |

### `--test cli_modes`

| Test                                      |
| ----------------------------------------- |
| `croft_dry_run_binary_exits_zero`         |
| `dispatch_dry_run_succeeds_for_each_mode` |
| `help_lists_croft_loft_tray_modes`        |
| `loft_dry_run_binary_exits_zero`          |
| `parse_selects_croft_loft_tray`           |
| `tray_dry_run_binary_exits_zero`          |

### `--test cli_starts`

| Test                                        |
| ------------------------------------------- |
| `help_exits_zero`                           |
| `version_exits_zero_and_prints_pkg_version` |

### `--test config_roundtrip`

| Test                          |
| ----------------------------- |
| `cli_override_wins_over_file` |
| `missing_file_loads_defaults` |
| `save_and_reload_roundtrip`   |

### `--test metadata_concurrent`

| Test                                             |
| ------------------------------------------------ |
| `concurrent_put_get_without_panic_or_corruption` |
| `last_write_wins_for_same_key`                   |

### `--test queues_stubs`

| Test                          |
| ----------------------------- |
| `activity_pause_resume_abort` |
| `gathering_marks_fresh`       |
| `reckoning_fresh_to_ready`    |

### `--test tray_stub`

| Test                      |
| ------------------------- |
| `tray_dry_run_returns_ok` |

### `--test ui_plugin_stubs`

| Test                                          |
| --------------------------------------------- |
| `croft_dry_run_returns_ok`                    |
| `engagement_plugin_stub_engage_ok`            |
| `fingerprint_plugin_stub_returns_placeholder` |
| `loft_dry_run_returns_ok`                     |
| `similarity_plugin_stub_returns_score`        |
