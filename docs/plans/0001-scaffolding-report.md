# Epic 0001 — Scaffolding FINAL REPORT (MC)

> **SUPERSEDED for merge / UI acceptance** — see **§ REVISE outcomes** below.  
> Stub-only Croft/Loft + top-level `tray` CLI are **no longer** merge acceptance.  
> **Branch:** `epic/scaffolding`  
> **Historical green:** `cargo test` **25/25** @ scaffolding close ([0001-test-green-log.md](./0001-test-green-log.md))  
> **REVISE green:** `cargo test` **30/30** after Wave C (same log file, REVISE section)  
> **Push:** none (never pushed)

---

## REVISE outcomes (2026-09-27) — active

Impl Waves A–C are green. Wave D aligns docs. **Do not merge until MC replies YES after seeing Croft and Loft startup work.** **Never push.**

| Item                    | Outcome                                                                 |
| ----------------------- | ----------------------------------------------------------------------- |
| Prior report            | **Superseded** for merge gate and UI DoD                                |
| Stub-only UI acceptance | **Invalidated** — dry-run-only success is not enough                    |
| Tray CLI mode           | **Removed** — no `mcload tray`; tray under Loft not required yet        |
| FrankenTUI              | **Starts** — Croft TTY + Loft Web (ftui 0.7.x; `frankentui` default-on) |
| LF hygiene              | **Done** — `.gitattributes` + renormalize (`639245f`)                   |
| Suite                   | **30 passed / 0 failed** after Wave C                                   |
| Schedule                | [0001-revise-workload-split.md](./0001-revise-workload-split.md)        |
| Merge ask               | **Do not ask** until MC manually confirms Croft + Loft start            |

### REVISE commits (`epic/scaffolding`)

| Hash               | Subject                                               | Wave       |
| ------------------ | ----------------------------------------------------- | ---------- |
| `639245f`          | chore: enforce LF via gitattributes and renormalize   | A / R-WS-1 |
| `fc57fac`          | docs: revise epic 0001 for real FrankenTUI startup    | plan prep  |
| `593f42d`          | refactor: remove tray as top-level CLI mode           | B / R-WS-2 |
| `c4b6e17`          | test: require real Croft/Loft startup contracts (RED) | B / R-WS-4 |
| `406b59c`          | feat: wire FrankenTUI Croft TTY and Loft Web startup  | C / R-WS-3 |
| *(pending parent)* | docs: align README and plans with 0001 revise         | D / R-WS-5 |

### Behavior (post–Wave C)

| Mode / seam                      | Behavior                                                                |
| -------------------------------- | ----------------------------------------------------------------------- |
| `cargo run -- croft`             | Interactive FrankenTUI TTY; needs real TTY; else clear `TtyUnavailable` |
| `cargo run -- loft`              | Serves HTML; prints `http://127.0.0.1:PORT`; Ctrl+C to stop             |
| `MCLOAD_STARTUP_PROBE=1 … croft` | Non-blocking: `frankentui-tty ready=true`                               |
| `MCLOAD_STARTUP_PROBE=1 … loft`  | Non-blocking: `frankentui-web ready=true` + listen URL                  |
| `… --dry-run`                    | Test seam only — **not** merge acceptance                               |

### Paste-ready commands for MC (confirm Croft + Loft)

**Do not merge until MC replies YES after seeing startup work. Never push.**

```bash
# From repo root (Linux / WSL / devcontainer)
cargo build
cargo run -- croft          # needs interactive TTY; press q to quit
cargo run -- loft           # note printed URL; open in browser; Ctrl+C to stop

# Non-blocking probes (CI seam — not the merge bar alone)
MCLOAD_STARTUP_PROBE=1 cargo run -- croft
MCLOAD_STARTUP_PROBE=1 cargo run -- loft
```

PowerShell (Windows host) equivalents:

```powershell
cargo build
cargo run -- croft
cargo run -- loft
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- croft
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- loft
```

Optional suite check:

```bash
cargo test
cargo run -- --help          # must list croft/loft; must NOT list tray
cargo run -- --version
```

---

## 1. What was done (scaffolding scope — historical)

Single-crate Rust binary `mcload` stood up from empty repo → TDD Waves A–E, MIT license, README, Rust dev-container, cross-compile docs only.

| Area             | Delivered (historical)                                                    |
| ---------------- | ------------------------------------------------------------------------- |
| Binary / CLI     | clap `--help` / `--version`; subcommands `croft` / `loft` / `tray`        |
| Config           | YAML load/save; merge defaults < user < project < CLI                     |
| Metadata         | `MetadataStore` + in-memory concurrent store (last-write-wins)            |
| Queues           | Activity Pause/Resume/Abort; Gathering `Fresh`; Reckoning `Fresh`→`Ready` |
| Plugins          | Engagement / fingerprint / similarity **stubs**                           |
| UI / tray        | Croft / Loft / tray **dry-run stubs** (no real FrankenTUI / native tray)  |
| Docs / container | README usage + `<details>`; `.devcontainer/`; Win/Linux/macOS notes       |
| Tests            | 25 passing (cli, config, metadata, queues, UI/plugin, tray)               |

**Out of scope (intentional then):** real FS profiling, SoT merge, plugin engines, FrankenTUI polish, tray menus, queue workers, CI publish, any `git push`.

**Invalidated by REVISE:** treating dry-run stubs + `mcload tray` as merge-ready UI delivery.

---

## 2. Key concepts tested / implemented (historical → revise)

| Concept                      | How proven (now)                                          |
| ---------------------------- | --------------------------------------------------------- |
| Binary starts cleanly        | `cli_starts` + smoke `--help` / `--version`               |
| Mode dispatch                | `cli_modes` — **croft / loft only** (no tray)             |
| Real UI startup              | `ui_startup` + `MCLOAD_STARTUP_PROBE=1`                   |
| Dry-run seam                 | `--dry-run` still exits 0; **not** merge bar              |
| Config round-trip + CLI wins | `config_roundtrip`                                        |
| Concurrent metadata access   | `metadata_concurrent` (stress + last-write-wins)          |
| Queue state markers          | lib + `queues_stubs`                                      |
| Plugin stubs                 | `ui_plugin_stubs`                                         |
| Optional features            | `frankentui` **default on**; no standalone `tray` feature |

---

## 3. Manual test commands (historical scaffolding — dry-run era)

> Prefer the **REVISE paste-ready block** above for post-revise verification.

### A — PowerShell host (Windows + optional GNU toolchain)

```powershell
# Repo root. Optional MinGW on PATH if using windows-gnu (adjust WinLibs path if needed):
$mingwBin = "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin"
if (Test-Path $mingwBin) { $env:Path = "$mingwBin;$env:USERPROFILE\.cargo\bin;$env:Path" }

# Full suite (default features)
cargo test

# Binary smoke (historical — tray + dry-run — superseded)
cargo run -- --help
cargo run -- --version
cargo run -- croft --dry-run
cargo run -- loft --dry-run
```

### B — Dev Container

```bash
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

## 5. Residual stubs (updated after REVISE)

| Stub                   | Behavior after Wave C                                  | Later                                     |
| ---------------------- | ------------------------------------------------------ | ----------------------------------------- |
| Croft (FrankenTUI TTY) | **Starts** (real TTY); clear error without TTY         | Polish → epic **0006**                    |
| Loft (FrankenTUI Web)  | **Starts**; prints local HTTP URL; serves minimal HTML | WASM host polish → epic **0006**          |
| Tray native UI         | No top-level CLI; under Loft **not required yet**      | Optional Loft capability when OS supports |
| Plugin engines         | placeholder engage / fingerprint / similarity          | 0003–0004                                 |
| Queue workers          | types + transitions only                               | 0002 / 0005                               |
| Metadata backend       | in-memory concurrent only                              | durable store when profiling lands        |

---

## 6. Merge permission (gated — do not ask yet)

> **Do not merge until MC replies YES after seeing Croft and Loft startup work.**  
> REVISE Waves A–C are green; Wave D docs aligned. Still **no push** unless separately ordered.  
> This report does **not** solicit merge.

| Answer  | Effect                                                       |
| ------- | ------------------------------------------------------------ |
| **YES** | (Only after manual UI confirm) PM may merge locally; no push |
| **NO**  | Branch stays open; say what must change                      |

---

## References

| Doc                                                              | Role                                    |
| ---------------------------------------------------------------- | --------------------------------------- |
| [MASTER.md](./MASTER.md)                                         | Epic board (REVISE green; await MC YES) |
| [0001-scaffolding.md](./0001-scaffolding.md)                     | Epic plan + REVISE DoD                  |
| [0001-revise-workload-split.md](./0001-revise-workload-split.md) | REVISE waves A–D                        |
| [0001-test-green-log.md](./0001-test-green-log.md)               | Historical 25/25 + REVISE 30/30         |
| [0001-workload-split.md](./0001-workload-split.md)               | Historical Waves A–E ownership          |
