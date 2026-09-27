# Epic 0001 REVISE — Workload Split (Verified)

> **Verdict:** VERIFIED (plan verifier 2026-09-27; waves A–D locked; schedule as written)  
> **Epic branch:** `epic/scaffolding` (revise on same epic; do not open 0002)  
> **Supersedes for revise scope:** tray-as-CLI-mode + dry-run-only UI acceptance from [0001-scaffolding.md](./0001-scaffolding.md) / [0001-workload-split.md](./0001-workload-split.md)  
> **Splitter / lock date:** 2026-09-27  
> **Verifier date:** 2026-09-27  
> **Out of scope (unchanged):** real dedup, full queue workers, push, production Loft WASM polish, crates.io publish  
> **Merge gate:** MC manual Croft + Loft start confirm — do **not** ask to merge until YES. Never push.

This file is the **REVISE** workload record for PM scheduling. Original scaffolding split remains historical; agents execute **R-WS-*** only.

**Hard ordering rules (MC):**

1. **LF** lands first as an exclusive tree normalize (Wave A).
2. **Tray CLI removal** before or **with** CLI/test updates that mention `tray`.
3. **RED** real-startup contract tests committed **before** FrankenTUI implementation.
4. FrankenTUI impl (Wave C) only after Wave B RED commit.
5. Docs last (Wave D).

---

## Corrected workstream table

| WS     | Name                                   | Blocks / depends on                     | Primary deliverables                                                                                                                                                                    |
| ------ | -------------------------------------- | --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R-WS-1 | LF line endings + local git hygiene    | Blocks all other revise edits           | `.gitattributes` (`* text=auto eol=lf` + binary exceptions); local `core.autocrlf=false` and/or `core.eol=lf`; `git add --renormalize .`; **own commit**                                |
| R-WS-2 | Remove tray CLI mode everywhere        | After R-WS-1                            | Delete `src/tray.rs`; drop standalone `tray` feature/launch path; strip clap `Tray` / `LaunchMode::Tray` / dispatch; fix `lib.rs` / `main.rs`; no `mcload tray`                         |
| R-WS-4 | Tests — drop tray; RED real-startup    | After or **with** R-WS-2; before R-WS-3 | Delete `tests/tray_stub.rs`; rewrite `tests/cli_modes.rs` (croft/loft only); commit RED tests for **real** Croft TTY + Loft Web startup (dry-run seam OK; dry-run-only ≠ acceptance)   |
| R-WS-3 | Real FrankenTUI startup (Croft + Loft) | After R-WS-4 RED commit                 | crates.io `ftui` 0.7.x; Croft TTY starts; Loft Web starts; **optional tray under Loft** when OS supports (background + tray) — **not** a separate CLI mode                               |
| R-WS-5 | Docs / README / plans alignment        | After R-WS-3 (preferred)                | README, scaffolding plan/DoD, report/logs, MASTER, prompt-history — modes = Croft+Loft; tray only as Loft-owned capability; LF + startup contracts documented                            |

---

## Parallel waves (locked A–D)

| Wave | Workstreams                              | Valid? | Rule                                                                                                                       |
| ---- | ---------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------- |
| A    | R-WS-1 only                              | Yes    | Exclusive normalize; **no other agent** edits text files until LF commit lands                                             |
| B    | R-WS-2 → R-WS-4 (sequential / one agent) | Yes    | Tray removal **before/with** CLI test updates; **commit RED** startup tests at end of B — do not ‖ on `cli.rs` vs tests    |
| C    | R-WS-3 only                              | Yes    | FrankenTUI impl **after** Wave B RED commit; tray support (if any) lives **inside Loft**, not a new top-level CLI mode     |
| D    | R-WS-5 only                              | Yes    | Docs last so README/plans match green real-startup + no tray CLI                                                           |

### Invalid parallels (do not schedule)

| Pair            | Why invalid                                                       |
| --------------- | ----------------------------------------------------------------- |
| R-WS-1 ‖ anyone | Renormalize rewrites CRLF across the tree → merge fights          |
| R-WS-2 ‖ R-WS-4 | Both need clap/help/mode assertions consistent; race on green/RED |
| R-WS-3 ‖ R-WS-4 | Impl before committed RED violates MC revise rule                 |
| R-WS-3 ‖ R-WS-2 | Both edit `cli.rs` / `Cargo.toml`                                 |
| R-WS-5 ‖ R-WS-3 | Prefer Wave D after C so docs match green behavior                |

---

## Coverage matrix (MC revise requirement → R-WS)

| #   | Requirement                                                                                 | WS covering it                         |
| --- | ------------------------------------------------------------------------------------------- | -------------------------------------- |
| 1   | LF: `.gitattributes` + local `core.autocrlf false` / `core.eol lf` + normalize + own commit | R-WS-1                                 |
| 2   | Remove tray CLI mode everywhere (clap, tests, docs, `src/tray.rs`, standalone feature)      | R-WS-2, R-WS-4, R-WS-5                 |
| 3   | Real FrankenTUI startup — Croft (TTY) + Loft (Web; optional tray under Loft)                | R-WS-3                                 |
| 4   | Tests: remove tray tests; RED real-startup contracts (not dry-run-only)                     | R-WS-4                                 |
| 5   | Docs / README / plans alignment                                                             | R-WS-5                                 |
| 6   | Merge only after MC manual Croft + Loft confirm; never push; do not ask merge until YES     | Process (after Wave D + MC YES)        |

---

## Module / file ownership (conflict avoidance)

| Path / area                               | Owning WS | Notes                                                                                                      |
| ----------------------------------------- | --------- | ---------------------------------------------------------------------------------------------------------- |
| `.gitattributes` (new)                    | R-WS-1    | `* text=auto eol=lf` + binary exceptions                                                                   |
| **Entire tree renormalize**               | R-WS-1    | Wave A exclusive; no parallel writers                                                                      |
| Local git config (not committed)          | R-WS-1    | Apply on agent machine in A; document in README via R-WS-5                                                 |
| `Cargo.toml` — remove standalone `tray`   | R-WS-2    | Do not add ftui deps here                                                                                  |
| `Cargo.toml` — `frankentui` / ftui 0.7.x  | R-WS-3    | After B; sole owner of UI dep pins in Wave C                                                               |
| `src/tray.rs`                             | R-WS-2    | **Delete** in Wave B; do not recreate as CLI module                                                        |
| `src/lib.rs` (`pub mod tray`)             | R-WS-2    | Remove module export                                                                                       |
| `src/cli.rs` — drop `Tray` mode           | R-WS-2    | Wave B — modes remain `croft` \| `loft` only                                                               |
| `src/cli.rs` — Loft-only flags            | R-WS-3    | Prefer **no** new top-level tray argument. If a test-only seam is required, optional under `loft` and PM-lockable — default is Loft auto-tray when OS supports it |
| `src/main.rs`                             | R-WS-2    | Comment / dispatch surface only if needed; no tray mode                                                    |
| `src/ui/croft.rs`                         | R-WS-3    | Real TTY FrankenTUI startup (`ftui` / `ftui-tty`)                                                          |
| `src/ui/loft.rs`                          | R-WS-3    | Real Web FrankenTUI startup (`ftui-web`); optional tray/BG **hosted under Loft**                           |
| `src/ui/mod.rs`                           | R-WS-3    | Only if Croft/Loft API surface changes                                                                     |
| `tests/tray_stub.rs`                      | R-WS-4    | **Delete** (with or immediately after R-WS-2)                                                              |
| `tests/cli_modes.rs`                      | R-WS-4    | Drop tray assertions; help lists `croft`/`loft` only                                                       |
| `tests/ui_plugin_stubs.rs`                | R-WS-4    | Replace dry-run-only UI acceptance with RED real-startup contracts                                         |
| `tests/*startup*` (new)                   | R-WS-4    | New RED files owned exclusively by R-WS-4                                                                  |
| `tests/cli_starts.rs`                     | R-WS-4    | Touch only if help/version copy mentions tray                                                              |
| `README.md`                               | R-WS-5    | Modes, features, LF note, startup how-to                                                                   |
| `docs/plans/0001-scaffolding.md`          | R-WS-5    | Keep REVISE goals/DoD in sync after green (planning agent may pre-align; R-WS-5 final checkbox pass)       |
| `docs/plans/0001-scaffolding-report.md`   | R-WS-5    | Align report with revise outcomes                                                                          |
| `docs/plans/0001-test-*-log.md`           | R-WS-4 → R-WS-5 | R-WS-4 may append RED/GREEN rows; R-WS-5 final narrative sync                                        |
| `docs/plans/MASTER.md`                    | R-WS-5    | Epic 0001 summary (REVISE; real FTUI; tray CLI out; merge gate)                                            |
| `docs/prompt-history/0001-scaffolding.md` | R-WS-5    | Full REVISE history section                                                                                |
| `plugins/`, `config/`, `metadata/`, `queues/` | —     | **No revise ownership** — do not touch                                                                     |

### Tray-under-Loft (locked policy)

| Item                         | Decision                                                                 |
| ---------------------------- | ------------------------------------------------------------------------ |
| Top-level `mcload tray`      | **Forbidden** — remove from clap/help/tests/docs                         |
| `loft --tray` as required mode | **Not required** — do not invent a mandatory CLI flag for tray         |
| When OS supports tray        | Loft **may** run as background process with tray — capability of Loft    |
| Test seam                    | Prefer env/config/internal hooks; if a CLI flag is needed for testing, it is **optional**, nested under `loft`, and PM-lockable          |
| Feature flag                 | No standalone `feature = "tray"` launch path; tray code (if any) lives under Loft module / Loft-private helper |

### Suggested Wave B commit split (same agent)

| Order | Commit focus                                    | Expected test state                                     |
| ----- | ----------------------------------------------- | ------------------------------------------------------- |
| B1    | R-WS-2 + tray test deletion / `cli_modes` scrub | Green on remaining non-startup contracts                |
| B2    | R-WS-4 RED real-startup tests committed         | **RED** on new startup contracts; other suites still OK |

Wave C then implements until B2 is green. Never push.

---

## Commit / TDD order (PM-compatible)

| Order | Wave | WS     | Example commit subject                                                         |
| ----- | ---- | ------ | ------------------------------------------------------------------------------ |
| 1     | A    | R-WS-1 | `chore: enforce LF via gitattributes and renormalize`                          |
| 2     | B    | R-WS-2 | `refactor: remove tray as top-level CLI mode`                                  |
| 3     | B    | R-WS-4 | `test: require real Croft/Loft startup contracts` (RED commit)                 |
| 4     | C    | R-WS-3 | `feat: wire FrankenTUI Croft TTY and Loft Web startup` (+ tray under Loft if any) |
| 5     | D    | R-WS-5 | `docs: align README and plans with 0001 revise`                                |

Per agent: **failing tests → impl → green → commit** for R-WS-3; R-WS-4’s RED commit is intentionally failing until order 4.

**FrankenTUI deps (Wave C):** prefer crates.io **0.7.x** — `ftui`, `ftui-runtime`, `ftui-tty`, `ftui-web` (plus needed transitive `ftui-*`). Nightly if required. Minimal hello-tick UI is enough for “starts”.

---

## Smoke gate — revise

| Gate                              | Minimum WS      | Command / check                                                                            |
| --------------------------------- | --------------- | ------------------------------------------------------------------------------------------ |
| Tree is LF                        | R-WS-1          | `.gitattributes` present; new edits LF                                                     |
| Help has croft/loft, not tray     | R-WS-2 + R-WS-4 | `cargo run -- --help`                                                                      |
| Dry-run may remain as seam        | R-WS-2          | Optional; **must not** be the sole acceptance for UI DoD                                   |
| Real Croft / Loft startup         | R-WS-3 + R-WS-4 | Startup contract tests green; MC can manually `mcload croft` / `mcload loft`               |
| Docs match behavior               | R-WS-5          | README + scaffolding plan/DoD checkboxes                                                   |
| Merge ask                         | After MC YES    | Plans/reports must **not** solicit merge until MC confirms both UIs start                  |

---

## Compact notes for PM

1. **Schedule Wave A alone** — LF renormalize is an exclusive write lock on the tree.
2. **One agent for Wave B** (R-WS-2 → R-WS-4) — avoids `cli.rs` / `tests/cli_modes.rs` thrash; two commits preferred (tray gone green → RED startup).
3. **Do not start R-WS-3 until B2 RED is on the branch.**
4. **Tray returns only under Loft** (optional, OS-dependent); never restore `mcload tray`; do not require `loft --tray` as a mode.
5. **R-WS-5 is last** — full prompt-history / final report belong here.
6. Untouched modules (`config`, `metadata`, `queues`, `plugins`) stay out of revise agents’ file lists.
7. Original [0001-workload-split.md](./0001-workload-split.md) stays as scaffolding history; this file owns REVISE scheduling.
8. After green revise: wait for MC manual Croft + Loft confirm before any merge request.
