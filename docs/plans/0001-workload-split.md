# Epic 0001 — Workload Split (Verified)

> **Verdict:** NEEDS_CHANGES (corrected split below; do not use the raw splitter draft as-is)  
> **Epic branch:** `epic/scaffolding`  
> **Source plans:** [0001-scaffolding.md](./0001-scaffolding.md), [MASTER.md](./MASTER.md)  
> **Verifier date:** 2026-09-27  
> **Out of scope (unchanged):** real dedup, full queue workers, push, FrankenTUI polish / production Loft

This file is the **approved corrected** workload record for PM scheduling. The splitter draft was close but missed plugins, left queues ambiguous, and allowed an invalid parallel on `cli.rs`.

---

## Corrected workstream table

| WS    | Name                                | Blocks / depends on                 | Plan steps     | Primary deliverables                                                                                         |
|-------|-------------------------------------|-------------------------------------|----------------|--------------------------------------------------------------------------------------------------------------|
| WS-1  | Cargo skeleton + repo hygiene       | Blocks all                          | 0, 1           | `.gitignore`, MIT `LICENSE`, stub `README`, `rust-toolchain.toml`, `Cargo.toml`, `lib.rs`+`bin`, `error.rs`, clap `--help`/`--version`, `tests/cli_starts.rs` |
| WS-2  | CLI modes                           | WS-1                                | 2              | `cli.rs` subcommands `croft`/`loft`/`tray`; dry-run / stub dispatch; help lists modes                         |
| WS-2b | Queue stubs                         | WS-1 (parallel OK with WS-2 / WS-4) | 5              | `queues/` Activity Pause/Resume/Abort; Gathering `Fresh`; Reckoning `Fresh`→`Ready`; unit tests              |
| WS-3  | Config YAML + CLI overrides         | WS-1 + **WS-2** (cli flag wiring)   | 3              | `config.rs`; defaults < user < project < CLI; `tests/config_roundtrip.rs`                                    |
| WS-4  | Metadata concurrent store           | WS-1                                | 4              | `metadata/` trait + `ConcurrentMemoryStore`; `tests/metadata_concurrent.rs`                                  |
| WS-5  | Croft / Loft + plugin stubs         | WS-2                                | 6 (UI+plugins) | `ui/croft.rs`, `ui/loft.rs`, `plugins/stubs.rs`; default stubs; optional `frankentui` feature                |
| WS-6  | Tray stub                           | WS-2                                | 6 (tray)       | `tray.rs` behind `feature = "tray"`; stub entry from `tray` mode                                             |
| WS-7  | README + dev-container + cross docs | All prior WS complete               | 7, 8, 9        | Full README (`<details>` Dev Container / Tests / Cross-build); `.devcontainer/`; matrix notes                |

---

## Parallel waves (corrected)

| Wave | Workstreams         | Valid? | Rule                                                                        |
|------|---------------------|--------|-----------------------------------------------------------------------------|
| A    | WS-1 only           | Yes    | Must land first; binary must start (`cargo run -- --help` / `--version`)    |
| B    | WS-2 ‖ WS-4 ‖ WS-2b | Yes    | Avoid same-file edits; queues need no metadata impl                         |
| C    | WS-3                | Yes    | **After WS-2** — owns `--config` / `--project` / override flags on `cli.rs` |
| D    | WS-5 ‖ WS-6         | Yes    | Both need CLI mode dispatch from WS-2; plugins owned by WS-5 only           |
| E    | WS-7                | Yes    | Last; docs + container after code DoD is green                              |

### Why the splitter draft was adjusted

| Issue                         | Splitter draft           | Correction                                                        |
|-------------------------------|--------------------------|-------------------------------------------------------------------|
| Plugins unmapped              | Not listed               | WS-5 owns `plugins/stubs.rs`                                      |
| Queues vague                  | "WS-2 or WS-2b"          | Explicit **WS-2b**; Wave B parallel with WS-4                     |
| Hidden `cli.rs` conflict      | WS-2 ‖ WS-3 same wave    | WS-3 moves to **Wave C** after WS-2                               |
| LICENSE / hygiene late-only   | MIT only in WS-7         | MIT + `.gitignore` + stub README in **WS-1**; WS-7 expands README |
| "Binary starts" critical path | Implicit in WS-1         | WS-1 DoD must include green `cli_starts` + runnable help/version  |

---

## Coverage matrix (MC scaffolding requirement → WS)

| #   | Requirement (Goal / DoD)                                   | WS covering it                              |
|-----|------------------------------------------------------------|---------------------------------------------|
| 1   | Cargo project + `main` entry                               | WS-1                                        |
| 2   | CLI (clap) help/version + Croft / Loft / tray modes        | WS-1, WS-2                                  |
| 3   | FrankenTUI hooks for Croft + Loft (stubs OK)               | WS-5                                        |
| 4   | System tray icon stub                                      | WS-6                                        |
| 5   | YAML config persistence + CLI overrides                    | WS-3                                        |
| 6   | Metadata persistence API with concurrent access            | WS-4                                        |
| 7   | Queue stubs (Activity / Gathering / Reckoning + markers)   | WS-2b                                       |
| 8   | Dev-container (Rust)                                       | WS-7                                        |
| 9   | README + MIT LICENSE                                       | WS-1 (MIT+stub), WS-7 (README completeness) |
| 10  | Minimal TDD tests (starts, config, metadata concurrent)    | WS-1, WS-3, WS-4 (+ WS-2b unit tests)       |
| 11  | Cross-compile notes (Win / Linux x86_64 / macOS aarch64)   | WS-7                                        |
| —   | Plugin stub module (engagement / fingerprint / similarity) | WS-5                                        |
| —   | `error.rs` + `lib` surface for integration tests           | WS-1                                        |
| —   | Plan checkboxes / MASTER status / no push                  | PM process (all WS)                         |

Every Goal item 1–11 and every Definition-of-Done bullet maps to at least one WS.

---

## Module ownership (`src/` layout)

| Module / path                  | Owning WS     | Notes                                                    |
|--------------------------------|---------------|----------------------------------------------------------|
| `main.rs`                      | WS-1          | Dispatch hooks filled by WS-2 / WS-5 / WS-6              |
| `lib.rs`                       | WS-1          | Testable library surface                                 |
| `error.rs`                     | WS-1          | Shared errors                                            |
| `cli.rs`                       | WS-2 → WS-3   | WS-2: modes; WS-3: config override flags only after WS-2 |
| `config.rs`                    | WS-3          |                                                          |
| `metadata/`                    | WS-4          |                                                          |
| `queues/`                      | WS-2b         |                                                          |
| `plugins/`                     | WS-5          |                                                          |
| `ui/croft.rs`, `ui/loft.rs`    | WS-5          | Stubs default; `frankentui` feature optional             |
| `tray.rs`                      | WS-6          | `feature = "tray"`                                       |
| `tests/cli_starts.rs`          | WS-1 (+ WS-2) | WS-2 extends assertions for modes                        |
| `tests/config_roundtrip.rs`    | WS-3          |                                                          |
| `tests/metadata_concurrent.rs` | WS-4          |                                                          |
| `.devcontainer/`               | WS-7          |                                                          |
| `README.md`                    | WS-1 → WS-7   | Stub in WS-1; full docs in WS-7                          |
| `LICENSE`                      | WS-1          | MIT                                                      |

---

## Commit / TDD order (PM-compatible)

Per WS agent, enforce: **plan touch (if needed) → failing tests → impl → green → commit**. Never push.

Suggested commit sequence on `epic/scaffolding` (aligns with plan Steps 0–9 and PM order plan → tests → impl → prompt-history → report):

| Order | WS    | Example commit subject (from plan)                                   |
|-------|-------|----------------------------------------------------------------------|
| 1     | WS-1  | `chore: init mcload binary with clap help/version` (+ hygiene files) |
| 2     | WS-2  | `feat: add croft/loft/tray CLI mode stubs`                           |
| 3     | WS-2b | `feat: activity/gathering/reckoning queue stubs`                     |
| 3'    | WS-4  | `feat: concurrent metadata store stub` (parallel with WS-2b OK)      |
| 4     | WS-3  | `feat: yaml config load/save with CLI overrides`                     |
| 5     | WS-5  | `feat: ui/plugin stubs for croft and loft` (split from plan Step 6)  |
| 5'    | WS-6  | `feat: tray stub` (parallel with WS-5 OK)                            |
| 6     | WS-7  | `chore: add rust devcontainer` then `docs: expand README…`           |

After each WS: agent prompt-history note + short report to PM. Epic closes only when DoD checkboxes in `0001-scaffolding.md` are green and `MASTER.md` still lists 0001 until explicitly closed.

---

## Smoke gate — "run binary to see it starts"

| Gate                       | Minimum WS                 | Command                                          |
|----------------------------|----------------------------|--------------------------------------------------|
| Binary exists and starts   | WS-1                       | `cargo run -- --help` / `cargo run -- --version` |
| Modes present (stub)       | WS-2                       | `cargo run -- croft --dry-run` (or chosen seam)  |
| Full default-feature tests | through WS-6 (+ WS-2b/3/4) | `cargo test`                                     |
| Container path             | WS-7                       | `cargo test` inside `.devcontainer`              |

WS-1 alone satisfies the critical "it starts" demo; epic DoD requires through WS-7.

---

## Compact notes for PM

1. **Do not schedule WS-2 ‖ WS-3** — both touch `cli.rs`; run WS-3 after WS-2 (Wave C).
2. **Pin queues as WS-2b** in Wave B with WS-4; do not bury inside WS-2 unless a single agent owns both and still commits queue work separately for TDD clarity.
3. **Assign plugins to WS-5** with Croft/Loft; tray stays WS-6 in parallel.
4. **Put MIT + `.gitignore` in WS-1**; WS-7 is README expansion + dev-container + cross-compile docs only.
5. **WS-1 is the start-binary gate**; keep its tests-first clap help/version commit atomic before parallel fan-out.
6. Out-of-scope list from splitter matches plan Non-goals — no change.
