# Epic 0001 — Scaffolding FINAL REPORT (MC)

> **Branch:** `epic/scaffolding` @ `f5f5c7a`  
> **Date:** 2026-09-27  
> **Suite:** `cargo test` **25/25 GREEN** ([0001-test-green-log.md](./0001-test-green-log.md))  
> **Push:** none (never pushed)

---

## 1. What was done (scaffolding scope)

Single-crate Rust binary `mcload` stood up from empty repo → TDD Waves A–E, MIT license, README, Rust dev-container, cross-compile docs only.

| Area              | Delivered                                                                 |
| ----------------- | ------------------------------------------------------------------------- |
| Binary / CLI      | clap `--help` / `--version`; subcommands `croft` / `loft` / `tray`        |
| Config            | YAML load/save; merge defaults < user < project < CLI                     |
| Metadata          | `MetadataStore` + in-memory concurrent store (last-write-wins)            |
| Queues            | Activity Pause/Resume/Abort; Gathering `Fresh`; Reckoning `Fresh`→`Ready` |
| Plugins           | Engagement / fingerprint / similarity **stubs**                           |
| UI / tray         | Croft / Loft / tray **dry-run stubs** (no real FrankenTUI / native tray)  |
| Docs / container   | README usage + `<details>`; `.devcontainer/`; Win/Linux/macOS notes       |
| Tests             | 25 passing (cli, config, metadata, queues, UI/plugin, tray)               |

**Out of scope (intentional):** real FS profiling, SoT merge, plugin engines, FrankenTUI polish, tray menus, queue workers, CI publish, any `git push`.

---

## 2. Key concepts tested / implemented

| Concept                      | How proven                                                 |
| ---------------------------- | ---------------------------------------------------------- |
| Binary starts cleanly        | `cli_starts` + smoke `--help` / `--version`                |
| Mode dispatch + dry-run seam | `cli_modes` + `croft`/`loft`/`tray --dry-run` exit 0       |
| Config round-trip + CLI wins | `config_roundtrip`                                         |
| Concurrent metadata access   | `metadata_concurrent` (stress + last-write-wins)           |
| Queue state markers          | lib + `queues_stubs`                                       |
| UI / plugin / tray stubs     | `ui_plugin_stubs`, `tray_stub`                             |
| Optional features reserved   | `tray`, `frankentui` (default off; stubs always callable)  |

---

## 3. Manual test commands (paste-ready)

### A — PowerShell host (Windows + optional GNU toolchain)

```powershell
# Repo root. Optional MinGW on PATH if using windows-gnu (adjust WinLibs path if needed):
$mingwBin = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin"
if (Test-Path $mingwBin) { $env:Path = "$mingwBin;$env:USERPROFILE\.cargo\bin;$env:Path" }

# Full suite (default features)
cargo test
# Or pinned gnu toolchain used in green log:
# cargo +stable-x86_64-pc-windows-gnu test --no-fail-fast

# Binary smoke
cargo run -- --help
cargo run -- --version
cargo run -- croft --dry-run
cargo run -- loft --dry-run
cargo run -- tray --dry-run

# Focused
cargo test --test cli_starts -- --nocapture
cargo test --test config_roundtrip -- --nocapture
cargo test --test metadata_concurrent -- --nocapture
```

### B — Dev Container

```powershell
# IDE: Dev Containers → Reopen in Container, then:
cargo test
cargo run -- --help
cargo run -- --version
cargo run -- croft --dry-run
cargo run -- loft --dry-run
cargo run -- tray --dry-run

# Or CLI from host:
devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . cargo test
```

---

## 4. Binary smoke (verifier re-check 2026-09-27)

| Command                   | Exit | Result                          |
| ------------------------- | ---- | ------------------------------- |
| `mcload --version`        | 0    | `mcload 0.1.0`                  |
| `mcload --help`           | 0    | lists `croft` / `loft` / `tray` |
| `mcload croft --dry-run`  | 0    | stub OK (no TTY)                |
| `mcload loft --dry-run`   | 0    | stub OK (no web UI)             |
| `mcload tray --dry-run`   | 0    | stub OK (no native tray)        |

**Smoke verdict:** GREEN

---

## 5. Residual stubs (later epics)

| Stub                         | Behavior now                                      | Later                                             |
| ---------------------------- | ------------------------------------------------- | ------------------------------------------------- |
| Croft (FrankenTUI TTY)       | `--dry-run` → Ok; else `NotImplemented`           | Epic 0006 / `feature = "frankentui"`              |
| Loft (FrankenTUI Web/WASM)   | same                                              | Epic 0006 / `feature = "frankentui"`              |
| Tray native UI               | dry-run Ok; else `NotImplemented`; `tray` feature | Activity/tray UX (0005+)                          |
| Plugin engines               | placeholder engage / fingerprint / similarity     | 0003–0004                                         |
| Queue workers                | types + transitions only                          | 0002 / 0005                                       |
| Metadata backend             | in-memory concurrent only                         | durable store when profiling lands                |

---

## 6. DECISION NEEDED — merge permission

Scaffolding DoD is met on **`epic/scaffolding`**. Plans mark epic **DONE**; `MASTER.md` still lists **0001 as CURRENT** until explicitly closed.

**MC: may PM merge `epic/scaffolding` → `main`?**

| Answer  | Effect                                                               |
| ------- | -------------------------------------------------------------------- |
| **YES** | PM merges locally (still **no push** unless you separately order it) |
| **NO**  | Branch stays open; say what must change before merge                 |

Reply with an explicit **YES** or **NO**. PM will merge only after MC says yes.

---

## References

| Doc                                                         | Role                          |
| ----------------------------------------------------------- | ----------------------------- |
| [MASTER.md](./MASTER.md)                                    | Epic board (awaiting merge)   |
| [0001-scaffolding.md](./0001-scaffolding.md)                | Epic plan + DoD checkboxes    |
| [0001-test-green-log.md](./0001-test-green-log.md)          | 25/25 suite capture           |
| [0001-workload-split.md](./0001-workload-split.md)          | Waves A–E ownership           |
