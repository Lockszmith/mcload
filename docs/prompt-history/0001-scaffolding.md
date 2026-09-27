# Prompt history — Epic 0001 Scaffolding

> **Audience:** future agents continuing McLoad work  
> **Epic:** `0001` scaffolding (`epic/scaffolding`)  
> **Plans:** [0001-scaffolding.md](../plans/0001-scaffolding.md), [0001-workload-split.md](../plans/0001-workload-split.md), [0001-revise-workload-split.md](../plans/0001-revise-workload-split.md), [MASTER.md](../plans/MASTER.md)  
> **Hard rule:** **never push** (no `git push`, no force-push, no remote publish unless a human explicitly overrides outside plan process)

This note preserves the agentic process, wave schedule, TDD loop, commits, and locked decisions for the scaffolding epic.

---

## REVISE process (Waves A–D) — 2026-09-27

MC reopened epic 0001 after scaffolding close: stub-only Croft/Loft + top-level `tray` CLI are **not** merge acceptance. Schedule: [0001-revise-workload-split.md](../plans/0001-revise-workload-split.md).

### MC revise intent

| #   | Requirement                                                                             |
| --- | --------------------------------------------------------------------------------------- |
| 1   | LF: `.gitattributes` + local eol hygiene + renormalize commit                           |
| 2   | Remove top-level `mcload tray` everywhere; tray (if any) lives under Loft only          |
| 3   | Real FrankenTUI startup — Croft TTY + Loft Web (crates.io ftui 0.7.x)                   |
| 4   | RED startup contracts before impl; dry-run may remain as seam only                      |
| 5   | Docs / README / plans / prompt-history alignment                                        |
| 6   | Merge only after MC manual Croft + Loft YES; **never push**; do not ask merge until YES |

### Waves executed

| Wave | WS           | Outcome                                                                   | Commit(s)  |
| ---- | ------------ | ------------------------------------------------------------------------- | ---------- |
| A    | R-WS-1       | `.gitattributes` LF + renormalize; local `core.autocrlf=false` / `eol=lf` | `639245f`  |
| —    | plan prep    | Docs revise epic plan / MASTER / split for FTUI+tray lock                 | `fc57fac`  |
| B    | R-WS-2       | Tray CLI / module / feature launch path removed                           | `593f42d`  |
| B    | R-WS-4 (RED) | Real Croft/Loft startup contracts committed failing                       | `c4b6e17`  |
| C    | R-WS-3       | ftui 0.7 wired; Croft TTY + Loft Web start; suite **30/30**               | `406b59c`  |
| D    | R-WS-5       | README / plans / report / green-log / this history aligned                | *(parent)* |

### Locked decisions **override** (scaffolding → REVISE)

| Decision     | Scaffolding (historical)                  | REVISE (locked now)                                                               |
| ------------ | ----------------------------------------- | --------------------------------------------------------------------------------- |
| FrankenTUI   | Deferred stubs; optional `frankentui` off | **Real startup** via crates.io **ftui 0.7.x**; `frankentui` **default on**        |
| Tray         | Top-level `mcload tray` dry-run stub      | **No tray CLI**; optional capability under Loft later (not required for 0001 DoD) |
| Dry-run      | Mode success path for UI acceptance       | **Test seam only** — not merge / UI acceptance                                    |
| Merge        | Scaffolding DoD → ask MC                  | Await **MC manual Croft + Loft YES**; do not solicit merge until then; never push |
| Line endings | Unspecified                               | LF via `.gitattributes` + local git eol                                           |

Historical “Decisions locked for scaffolding” below remain the audit trail for Waves A–E; treat the override table above as authoritative for merge.

### REVISE smoke / probe

```bash
MCLOAD_STARTUP_PROBE=1 cargo run -- croft   # frankentui-tty ready=true
MCLOAD_STARTUP_PROBE=1 cargo run -- loft    # frankentui-web ready=true + listen URL
cargo run -- croft                          # interactive TTY; TtyUnavailable without TTY
cargo run -- loft                           # prints http://127.0.0.1:PORT
```

### REVISE key commits

| Abbrev    | Full hash                                  | Subject                                              |
| --------- | ------------------------------------------ | ---------------------------------------------------- |
| `639245f` | `639245fbf71ec216d7ae2a83283e06af075f82ee` | chore: enforce LF via gitattributes and renormalize  |
| `fc57fac` | `fc57fac78d2fcba81694da00d6f344472fcd290e` | docs: revise epic 0001 for real FrankenTUI startup   |
| `593f42d` | `593f42d154472e0b81f12740eaffc99c6e1756cf` | refactor: remove tray as top-level CLI mode          |
| `c4b6e17` | `c4b6e17a9d6ebbf4ef88922a9d0fdb7c89df2086` | test: require real Croft/Loft startup contracts      |
| `406b59c` | `406b59c2cd718febb75d89c06a3c92699f202870` | feat: wire FrankenTUI Croft TTY and Loft Web startup |

Wave D docs commit is owned by the parent agent.

**Never push.** Do not ask to merge until MC YES after seeing Croft + Loft startup.

---

## MC request (scaffolding)

Stand up a compilable, testable single-crate Rust binary `mcload` ("There can be only one!") covering:

| #   | Requirement                                              |
| --- | -------------------------------------------------------- |
| 1   | Cargo project + `main` entry                             |
| 2   | CLI (clap) help/version + Croft / Loft / tray modes      |
| 3   | FrankenTUI hooks for Croft + Loft (stubs OK)             |
| 4   | System tray icon stub                                    |
| 5   | YAML config persistence + CLI overrides                  |
| 6   | Metadata persistence API with concurrent access          |
| 7   | Queue stubs (Activity / Gathering / Reckoning + markers) |
| 8   | Dev-container (Rust)                                     |
| 9   | README + MIT LICENSE                                     |
| 10  | Minimal TDD tests (starts, config, metadata concurrent)  |
| 11  | Cross-compile notes (Win / Linux x86_64 / macOS aarch64) |

Non-goals stayed out of scope: real dedup / SoT merge, full queue workers, production FrankenTUI / Loft WASM, production tray, crates.io publish, CI beyond local docs, and **any push**.

---

## Workload split — Waves A–E (corrected)

Source of truth: [0001-workload-split.md](../plans/0001-workload-split.md). The raw splitter draft was close but **not** scheduled as-is.

| Wave | Workstreams         | Rule                                                            |
| ---- | ------------------- | --------------------------------------------------------------- |
| A    | WS-1 only           | Cargo skeleton + clap help/version; binary must start           |
| B    | WS-2 ‖ WS-4 ‖ WS-2b | CLI modes ‖ metadata store ‖ queue stubs; avoid same-file edits |
| C    | WS-3                | Config YAML + CLI overrides **after** WS-2 (`cli.rs` ownership) |
| D    | WS-5 ‖ WS-6         | Croft/Loft + plugin stubs ‖ tray stub                           |
| E    | WS-7                | README expansion + Rust dev-container + cross-build notes       |

### Corrections vs splitter draft

| Issue                    | Correction                                                    |
| ------------------------ | ------------------------------------------------------------- |
| Plugins unmapped         | WS-5 owns `plugins/stubs.rs`                                  |
| Queues vague             | Explicit **WS-2b** in Wave B with WS-4                        |
| Hidden `cli.rs` conflict | Do **not** run WS-2 ‖ WS-3; WS-3 is Wave C after WS-2         |
| Hygiene late-only        | MIT + `.gitignore` + stub README in WS-1; WS-7 expands README |
| "Binary starts" gate     | WS-1 DoD includes green `cli_starts` + runnable help/version  |

---

## Agentic process loop

Process constraints from MASTER / epic plan:

1. **Plan** — verified epic plan + corrected workload split on branch  
2. **Verify** — PM/verifier approval of waves before fan-out  
3. **RED tests** — failing scaffolding contract suite first  
4. **Impl waves** — A → B → C → D → E (parallel only where wave table allows)  
5. **GREEN** — full suite pass + binary smoke; record RED/GREEN logs under `docs/plans/`  
6. **Prompt history** — this file; then reports / epic close as PM directs  
7. **Never push**

Per workstream: plan touch (if needed) → failing tests → impl → green → commit. One coherent commit per completed TDD step / WS deliverable.

| Phase        | Artifact / gate                                                        |
| ------------ | ---------------------------------------------------------------------- |
| Plan         | `docs/plans/0001-scaffolding.md`, `MASTER.md`                          |
| Verify split | `docs/plans/0001-workload-split.md` (corrected waves)                  |
| RED          | `8d4a06d` + `docs/plans/0001-test-red-log.md` (25 failed / 0 passed)   |
| Impl         | Waves A–E feature commits below                                        |
| GREEN        | `6c16db5` + `docs/plans/0001-test-green-log.md` (25 passed / 0 failed) |

---

## Key commits (on `epic/scaffolding`)

Hashes from `git log` at prompt-history authoring time:

| Hash      | Subject                                                      | Wave / role            |
| --------- | ------------------------------------------------------------ | ---------------------- |
| `db9d5c6` | docs: add verified scaffolding epic plan                     | Plan                   |
| `82ea51a` | docs: correct scaffolding workload split waves               | Verify (corrected A–E) |
| `8d4a06d` | test: add failing scaffolding contract suite                 | RED                    |
| `f49ec83` | feat: wire clap so help and version start cleanly            | Wave A (WS-1)          |
| `9a25a6e` | feat: dispatch croft loft and tray CLI modes                 | Wave B (WS-2)          |
| `eadb827` | feat: implement concurrent in-memory metadata store          | Wave B (WS-4)          |
| `2744eda` | feat: implement activity gathering and reckoning queue stubs | Wave B (WS-2b)         |
| `a9bc090` | feat: persist YAML config with CLI overrides                 | Wave C (WS-3)          |
| `f7ce8ac` | feat: stub tray dry-run success path                         | Wave D (WS-6)          |
| `a5e1841` | feat: stub Croft Loft and plugin dry-run paths               | Wave D (WS-5)          |
| `154935e` | docs: add README usage and Rust dev-container                | Wave E (WS-7)          |
| `6c16db5` | test: record scaffolding suite green log                     | GREEN record           |

Full SHAs (abbreviated above to 7 chars):

| Abbrev    | Full hash                                  |
| --------- | ------------------------------------------ |
| `db9d5c6` | `db9d5c6601ba84ad8decd4bb7d6595b817dd84b1` |
| `82ea51a` | `82ea51a299702710aec0a89fd01ee9080cc5f733` |
| `8d4a06d` | `8d4a06db1d42df2a62288a3ed31f5347fa8550d7` |
| `f49ec83` | `f49ec83aacbde81cca53c0c47132265774389e62` |
| `9a25a6e` | `9a25a6e2a39f340f2b3ec47f8aaee2ed7739618a` |
| `eadb827` | `eadb827371a737d5226fcd47c9db066b8c4f9cdf` |
| `2744eda` | `2744edac97807a0c23000e46bba3ac7181bbd49f` |
| `a9bc090` | `a9bc090acb5db2b24d06a83e93b91676cf052a6d` |
| `f7ce8ac` | `f7ce8acb636ab05de046a6541b7cf9ccc37b070e` |
| `a5e1841` | `a5e184167022c9c57362801e6e5c41e27b676645` |
| `154935e` | `154935ed1bc3e3df86239f4372a0c6f5b376ae25` |
| `6c16db5` | `6c16db529a52300de15c1d3f367454059be6fb89` |

---

## Decisions locked for scaffolding

| Decision             | Choice                                                                  | Why / where                                                               |
| -------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| CLI framework        | **clap** (derive)                                                       | Help/version + subcommands; industry default (`src/cli.rs`)               |
| Metadata concurrency | **In-memory `Arc<RwLock<…>>`** (`ConcurrentMemoryStore`)                | Concurrent API without sled/sqlite in epic 0001 (`src/metadata/store.rs`) |
| Dry-run seam         | **`--dry-run` short-circuits in `cli::dispatch`**                       | Returns `Ok(())` before UI/tray; tests smoke without opening backends     |
| FrankenTUI           | **Deferred stubs** (`ui/croft.rs`, `ui/loft.rs`); optional `frankentui` | Compile/test without full ftui stack; real wiring is later epic           |
| Tray                 | **Dry-run stub** (`tray::run`); feature-gated real path later           | Mode succeeds under dry-run / stub without OS tray backend                |
| Crate layout         | Single crate `lib` + `bin`                                              | Integration tests import modules without always spawning                  |
| Remote ops           | **Never push**                                                          | Process constraint for all WS agents and PM                               |

### Dry-run short-circuit (explicit)

`cli::dispatch(mode, dry_run)` returns immediately on `dry_run == true`. Non-dry-run forwards to `ui::croft` / `ui::loft` / `tray`. UI/tray modules themselves also accept a dry-run flag for stub success paths used by contract tests.

---

## Pointers for the next agent

| Path                                       | Use                                         |
| ------------------------------------------ | ------------------------------------------- |
| `docs/plans/MASTER.md`                     | Epic status / upcoming IDs                  |
| `docs/plans/0001-scaffolding.md`           | Goal, DoD, architecture, open questions     |
| `docs/plans/0001-workload-split.md`        | Historical WS ownership and waves A–E       |
| `docs/plans/0001-revise-workload-split.md` | REVISE waves A–D / R-WS schedule            |
| `docs/plans/0001-scaffolding-report.md`    | MC report + paste-ready Croft/Loft commands |
| `docs/plans/0001-test-red-log.md`          | RED baseline (scaffolding 25 fail)          |
| `docs/plans/0001-test-green-log.md`        | GREEN: scaffolding 25 + REVISE 30           |
| `docs/prompt-history/0001-scaffolding.md`  | This process record                         |

Do not re-litigate clap vs alternatives, RwLock vs DB, or dry-run vs env-only seams for scaffolding follow-ups unless a new epic plan explicitly revisits them. For REVISE, treat the **override** table (real FTUI + no tray CLI) as authoritative. Await MC YES on Croft+Loft before any merge ask. **Never push.**
