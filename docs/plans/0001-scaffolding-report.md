# Epic 0001 — Scaffolding FINAL REPORT (MC)

> **SUPERSEDED for merge / UI acceptance** — see **§ REVISE** below.  
> Stub-only Croft/Loft + top-level `tray` CLI are **no longer** merge acceptance.  
> **Branch:** `epic/scaffolding`  
> **Historical green:** `cargo test` **25/25** @ scaffolding close ([0001-test-green-log.md](./0001-test-green-log.md))  
> **Push:** none (never pushed)

---

## REVISE (active — 2026-09-27)

| Item                    | Status                                                                 |
| ----------------------- | ---------------------------------------------------------------------- |
| Prior report            | **Superseded** for merge gate and UI DoD                               |
| Stub-only UI acceptance | **Invalidated** by MC — dry-run-only success is not enough             |
| Tray CLI mode           | **Must be removed** — tray (if any) lives under Loft only              |
| FrankenTUI              | **Must actually start** — Croft TTY + Loft Web (ftui 0.7.x crates.io)  |
| LF hygiene              | Required — own commit (R-WS-1)                                         |
| Schedule                | [0001-revise-workload-split.md](./0001-revise-workload-split.md)       |
| Merge ask               | **Do not ask** until MC manually confirms Croft + Loft start           |

### Paste-ready commands (for later R-WS-5 / MC confirm)

```powershell
# After REVISE Waves A–D green — MC manual confirm (not dry-run-only):
cargo test
cargo run -- --help          # must list croft/loft; must NOT list tray
cargo run -- --version
cargo run -- croft           # must start FrankenTUI TTY (or clear actionable error)
cargo run -- loft            # must start FrankenTUI Web UI
```

Final revise report narrative lands in R-WS-5; this header + historical sections below preserve the scaffolding audit trail.

---

## 1. What was done (scaffolding scope — historical)

Single-crate Rust binary `mcload` stood up from empty repo → TDD Waves A–E, MIT license, README, Rust dev-container, cross-compile docs only.

| Area            | Delivered (historical)                                                    |
| --------------- | ------------------------------------------------------------------------- |
| Binary / CLI    | clap `--help` / `--version`; subcommands `croft` / `loft` / `tray`        |
| Config          | YAML load/save; merge defaults < user < project < CLI                     |
| Metadata        | `MetadataStore` + in-memory concurrent store (last-write-wins)            |
| Queues          | Activity Pause/Resume/Abort; Gathering `Fresh`; Reckoning `Fresh`→`Ready` |
| Plugins         | Engagement / fingerprint / similarity **stubs**                           |
| UI / tray       | Croft / Loft / tray **dry-run stubs** (no real FrankenTUI / native tray)  |
| Docs / container | README usage + `<details>`; `.devcontainer/`; Win/Linux/macOS notes       |
| Tests           | 25 passing (cli, config, metadata, queues, UI/plugin, tray)               |

**Out of scope (intentional then):** real FS profiling, SoT merge, plugin engines, FrankenTUI polish, tray menus, queue workers, CI publish, any `git push`.

**Invalidated by REVISE:** treating dry-run stubs + `mcload tray` as merge-ready UI delivery.

---

## 2. Key concepts tested / implemented (historical)

| Concept                      | How proven                                                |
| ---------------------------- | --------------------------------------------------------- |
| Binary starts cleanly        | `cli_starts` + smoke `--help` / `--version`               |
| Mode dispatch + dry-run seam | `cli_modes` + `croft`/`loft`/`tray --dry-run` exit 0      |
| Config round-trip + CLI wins | `config_roundtrip`                                        |
| Concurrent metadata access   | `metadata_concurrent` (stress + last-write-wins)          |
| Queue state markers          | lib + `queues_stubs`                                      |
| UI / plugin / tray stubs     | `ui_plugin_stubs`, `tray_stub`                            |
| Optional features reserved   | `tray`, `frankentui` (default off; stubs always callable) |

---

## 3. Manual test commands (historical scaffolding — dry-run era)

> Prefer the **REVISE** paste-ready block above for post-revise verification.

### A — PowerShell host (Windows + optional GNU toolchain)

```powershell
# Repo root. Optional MinGW on PATH if using windows-gnu (adjust WinLibs path if needed):
$mingwBin = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin"
if (Test-Path $mingwBin) { $env:Path = "$mingwBin;$env:USERPROFILE\.cargo\bin;$env:Path" }

# Full suite (default features)
cargo test

# Binary smoke (historical — tray + dry-run)
cargo run -- --help
cargo run -- --version
cargo run -- croft --dry-run
cargo run -- loft --dry-run
cargo run -- tray --dry-run
```

### B — Dev Container

```powershell
cargo test
cargo run -- --help
cargo run -- --version
```

---

## 4. Binary smoke (verifier re-check 2026-09-27 — historical)

| Command                  | Exit | Result                          |
| ------------------------ | ---- | ------------------------------- |
| `mcload --version`       | 0    | `mcload 0.1.0`                  |
| `mcload --help`          | 0    | lists `croft` / `loft` / `tray` |
| `mcload croft --dry-run` | 0    | stub OK (no TTY)                |
| `mcload loft --dry-run`  | 0    | stub OK (no web UI)             |
| `mcload tray --dry-run`  | 0    | stub OK (no native tray)        |

**Historical smoke verdict:** GREEN for scaffolding stubs. **Not sufficient** for REVISE merge gate.

---

## 5. Residual stubs (updated target)

| Stub                       | Behavior at scaffolding close                         | REVISE / later                                              |
| -------------------------- | ----------------------------------------------------- | ----------------------------------------------------------- |
| Croft (FrankenTUI TTY)     | `--dry-run` → Ok; else `NotImplemented`               | **0001 REVISE:** must start (minimal UI OK)                 |
| Loft (FrankenTUI Web/WASM) | same                                                  | **0001 REVISE:** must start Web; optional tray under Loft   |
| Tray native UI             | top-level `tray` mode + feature                       | **Remove CLI mode**; tray only as Loft-owned capability     |
| Plugin engines             | placeholder engage / fingerprint / similarity         | 0003–0004                                                   |
| Queue workers              | types + transitions only                              | 0002 / 0005                                                 |
| Metadata backend           | in-memory concurrent only                             | durable store when profiling lands                          |
| UI polish                  | n/a                                                   | Epic **0006** (startup pulled into 0001 REVISE)             |

---

## 6. Merge permission (gated — do not ask yet)

> **Withdrawn for now.** Prior scaffolding DoD met stub acceptance only.  
> After REVISE Waves A–D are green, MC must **manually** confirm Croft and Loft start.  
> Plans will solicit merge **only after** that explicit YES. Still **no push** unless separately ordered.

| Answer  | Effect                                                               |
| ------- | -------------------------------------------------------------------- |
| **YES** | (Only after REVISE + manual UI confirm) PM merges locally; no push   |
| **NO**  | Branch stays open; say what must change                              |

---

## References

| Doc                                                           | Role                                      |
| ------------------------------------------------------------- | ----------------------------------------- |
| [MASTER.md](./MASTER.md)                                      | Epic board (CURRENT — REVISE)             |
| [0001-scaffolding.md](./0001-scaffolding.md)                  | Epic plan + REVISE DoD                    |
| [0001-revise-workload-split.md](./0001-revise-workload-split.md) | REVISE waves A–D                       |
| [0001-test-green-log.md](./0001-test-green-log.md)            | Historical 25/25 suite capture            |
| [0001-workload-split.md](./0001-workload-split.md)            | Historical Waves A–E ownership            |
