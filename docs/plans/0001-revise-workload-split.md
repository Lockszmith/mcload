# Epic 0001 REVISE — Workload Split (Verified)

> **Verdict:** VERIFIED schedule; Wave C **implemented** (submodule + WebHost + tray-under-Loft); **merge still gated** on MC Windows tray / Croft+Loft YES  
> **Execution:** Waves **A–B done**; Wave **C** code green in CI (`--no-tray` / quit seams); Wave **D** docs largely aligned; **do not ask merge** until MC YES  
> **Epic branch:** `epic/scaffolding` (revise on same epic; do not open 0002 / 0005 for this)  
> **Supersedes for revise scope:** tray-as-CLI-mode + dry-run-only UI acceptance + weak “starts” DoD from [0001-scaffolding.md](./0001-scaffolding.md) / [0001-workload-split.md](./0001-workload-split.md)  
> **Splitter / lock date:** 2026-09-27  
> **Verifier date:** 2026-09-27 (DoD harden same day); Wave C re-impl 2026-09-28 (frankentui submodule + loft_host)  
> **Out of scope (unchanged):** real dedup, full queue workers, push, Croft/Loft polish beyond shared min Model/App, crates.io publish  
> **Merge gate:** MC manual Croft + Loft confirm against **parity / `q` quit / tray** DoD — do **not** ask to merge until YES. Never push.

This file is the **REVISE** workload record for PM scheduling. Original scaffolding split remains historical; agents execute **R-WS-*** only.

**Hard ordering rules (MC):**

1. **LF** lands first as an exclusive tree normalize (Wave A).
2. **Tray CLI removal** before or **with** CLI/test updates that mention `tray`.
3. **RED** real-startup contract tests committed **before** FrankenTUI implementation.
4. FrankenTUI impl (Wave C) only after Wave B RED commit — and Wave C is **not closed** until parity + quit + tray DoD pass.
5. Docs last (Wave D) — only after Wave C re-closes.

---

## Corrected workstream table

| WS     | Name                                   | Blocks / depends on                     | Primary deliverables                                                                                                                                                                                                                                                                 |
| ------ | -------------------------------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| R-WS-1 | LF line endings + local git hygiene    | Blocks all other revise edits           | `.gitattributes` (`* text=auto eol=lf` + binary exceptions); local `core.autocrlf=false` and/or `core.eol=lf`; `git add --renormalize .`; **own commit**                                                                                                                             |
| R-WS-2 | Remove tray CLI mode everywhere        | After R-WS-1                            | Delete `src/tray.rs`; drop standalone `tray` feature/launch path; strip clap `Tray` / `LaunchMode::Tray` / dispatch; fix `lib.rs` / `main.rs`; no `mcload tray`                                                                                                                      |
| R-WS-4 | Tests — drop tray; RED real-startup    | After or **with** R-WS-2; before R-WS-3 | Delete `tests/tray_stub.rs`; rewrite `tests/cli_modes.rs` (croft/loft only); commit RED tests for **real** Croft TTY + Loft Web startup (dry-run seam OK; dry-run-only ≠ acceptance); extend/update contracts for parity + `q` quit as Wave C reopens                                    |
| R-WS-3 | Real FrankenTUI parity + quit + tray   | After R-WS-4 RED commit                 | crates.io `ftui` 0.7.x; **same** Model/App for Croft+Loft (name + TTY/Web only); `q` exits both (Loft stops host exit 0); Loft tray+BG **MUST** on tray-capable OS; `--no-tray` / `--verbose`; **not** a separate CLI mode; **not** `loft --tray` required                            |
| R-WS-5 | Docs / README / plans alignment        | After R-WS-3 **re-closes**              | README, scaffolding plan/DoD, report/logs, MASTER, prompt-history — modes = Croft+Loft; document `q` quit, Loft tray MUST, `--no-tray` for WSL/CI, `--verbose`; LF + startup/parity/quit contracts documented                                                                         |

---

## Parallel waves (locked A–D)

| Wave | Workstreams                              | Valid? | Executed?     | Rule                                                                                                                                          |
| ---- | ---------------------------------------- | ------ | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| A    | R-WS-1 only                              | Yes    | **Done**      | Exclusive normalize; **no other agent** edits text files until LF commit lands                                                                |
| B    | R-WS-2 → R-WS-4 (sequential / one agent) | Yes    | **Done**      | Tray removal **before/with** CLI test updates; **commit RED** startup tests at end of B — do not ‖ on `cli.rs` vs tests                       |
| C    | R-WS-3 only                              | Yes    | **Implemented** | Submodule frankentui + shared Model + loft_host `q` quit + tray-under-Loft (Win/macOS). MC Windows tray YES still required before merge ask. |
| D    | R-WS-5 only                              | Yes    | **In progress** | Docs aligned with submodule / `q` / `--no-tray`; final checkbox pass after MC YES                                                                 |

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

| #   | Requirement                                                                                              | WS covering it                  |
| --- | -------------------------------------------------------------------------------------------------------- | ------------------------------- |
| 1   | LF: `.gitattributes` + local `core.autocrlf false` / `core.eol lf` + normalize + own commit              | R-WS-1                          |
| 2   | Remove tray CLI mode everywhere (clap, tests, docs, `src/tray.rs`, standalone feature)                   | R-WS-2, R-WS-4, R-WS-5          |
| 3   | Shared FrankenTUI Model/App parity (Croft TTY + Loft Web; name + backend only)                           | R-WS-3                          |
| 4   | `q` quit both backends; Loft `q` stops Web host (exit 0)                                                 | R-WS-3                          |
| 5   | Loft tray+BG MUST on Windows / tray-capable OS; `--no-tray` / `--verbose`; no `loft --tray` required     | R-WS-3                          |
| 6   | Tests: remove tray-mode tests; RED/update real-startup + parity + quit contracts (not dry-run-only)      | R-WS-4                          |
| 7   | Docs / README / plans alignment with hardened DoD                                                        | R-WS-5                          |
| 8   | Merge only after MC manual Croft + Loft confirm against DoD; never push; do not ask merge until YES      | Process (after Wave D + MC YES) |

---

## Module / file ownership (conflict avoidance)

| Path / area                                   | Owning WS       | Notes                                                                                                                                                                      |
| --------------------------------------------- | --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.gitattributes` (new)                        | R-WS-1          | `* text=auto eol=lf` + binary exceptions                                                                                                                                   |
| **Entire tree renormalize**                   | R-WS-1          | Wave A exclusive; no parallel writers                                                                                                                                      |
| Local git config (not committed)              | R-WS-1          | Apply on agent machine in A; document in README via R-WS-5                                                                                                                 |
| `Cargo.toml` — remove standalone `tray`       | R-WS-2          | Do not add ftui deps here                                                                                                                                                  |
| `Cargo.toml` — `frankentui` / ftui 0.7.x      | R-WS-3          | After B; sole owner of UI dep pins in Wave C                                                                                                                               |
| `src/tray.rs`                                 | R-WS-2          | **Delete** in Wave B; do not recreate as CLI module                                                                                                                        |
| `src/lib.rs` (`pub mod tray`)                 | R-WS-2          | Remove module export                                                                                                                                                       |
| `src/cli.rs` — drop `Tray` mode               | R-WS-2          | Wave B — modes remain croft and loft only                                                                                                                                  |
| `src/cli.rs` — Loft flags                     | R-WS-3          | **`--no-tray`** (force foreground) and **`--verbose`** (allow CLI logs) under `loft`. **Do not** require `loft --tray`. Default = tray+BG when OS supports tray.           |
| `src/main.rs`                                 | R-WS-2          | Comment / dispatch surface only if needed; no tray mode                                                                                                                    |
| `src/ui/` shared Model/App                    | R-WS-3          | One FrankenTUI `Model`/`App`; Croft/Loft wrap backends only                                                                                                                |
| `src/ui/croft.rs`                             | R-WS-3          | TTY FrankenTUI; `q` exits process                                                                                                                                          |
| `src/ui/loft.rs`                              | R-WS-3          | Web FrankenTUI; `q` stops host (exit 0); tray+BG **MUST** when OS supports; quiet unless `--verbose`                                                                       |
| `src/ui/mod.rs`                               | R-WS-3          | Only if Croft/Loft API surface changes                                                                                                                                     |
| `tests/tray_stub.rs`                          | R-WS-4          | **Delete** (with or immediately after R-WS-2)                                                                                                                              |
| `tests/cli_modes.rs`                          | R-WS-4          | Drop tray-mode assertions; help lists croft/loft only; cover loft `--no-tray` / `--verbose` as needed                                                                      |
| `tests/ui_plugin_stubs.rs`                    | R-WS-4          | Replace dry-run-only UI acceptance with RED real-startup / parity / quit contracts                                                                                         |
| `tests/*startup*` (new)                       | R-WS-4          | New RED files owned exclusively by R-WS-4; update when Wave C DoD hardens                                                                                                  |
| `tests/cli_starts.rs`                         | R-WS-4          | Touch only if help/version copy mentions tray                                                                                                                              |
| `README.md`                                   | R-WS-5          | Modes, features, LF note, `q` quit, tray MUST, `--no-tray` for WSL/CI                                                                                                      |
| `docs/plans/0001-scaffolding.md`              | R-WS-5          | Keep REVISE goals/DoD in sync after green (planning agent may pre-align; R-WS-5 final checkbox pass)                                                                       |
| `docs/plans/0001-scaffolding-report.md`       | R-WS-5          | Align report with revise outcomes                                                                                                                                          |
| `docs/plans/0001-test-*-log.md`               | R-WS-4 → R-WS-5 | R-WS-4 may append RED/GREEN rows; R-WS-5 final narrative sync                                                                                                              |
| `docs/plans/MASTER.md`                        | R-WS-5          | Epic 0001 summary (REVISE; Wave C reopen; tray CLI out; hardened DoD; merge gate)                                                                                          |
| `docs/prompt-history/0001-scaffolding.md`     | R-WS-5          | Full REVISE history section                                                                                                                                                |
| `plugins/`, `config/`, `metadata/`, `queues/` | —               | **No revise ownership** — do not touch                                                                                                                                     |

### Tray-under-Loft (locked policy)

| Item                                | Decision                                                                                                                                      |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| Top-level `mcload tray`             | **Forbidden** — remove from clap/help/tests/docs                                                                                              |
| `loft --tray` as required mode      | **Forbidden as a requirement** — do not invent a mandatory enable flag; tray is default-on when OS supports it                                |
| When OS supports tray (Windows…)    | Loft **MUST** show tray icon and keep running in background after launch; **no** CLI output unless `--verbose`                                |
| When OS lacks tray / `--no-tray`    | Run web server in foreground; Ctrl+C allowed to exit; document `--no-tray` for WSL / headless CI                                              |
| Native Windows host                 | Default `mcload loft` **MUST** show tray icon — **FAIL** if missing                                                                           |
| Test / CI seam                      | Prefer `--no-tray` and/or env/config hooks under `loft`; never restore top-level tray mode                                                    |
| Feature flag                        | No standalone `feature = "tray"` launch path; tray code lives under Loft module / Loft-private helper                                         |
| Parked in epic 0005                 | **Invalid** — tray+BG under Loft is **0001 MUST**, not deferred                                                                               |

### Suggested Wave B commit split (same agent)

| Order | Commit focus                                    | Expected test state                                     |
| ----- | ----------------------------------------------- | ------------------------------------------------------- |
| B1    | R-WS-2 + tray test deletion / `cli_modes` scrub | Green on remaining non-startup contracts                |
| B2    | R-WS-4 RED real-startup tests committed         | **RED** on new startup contracts; other suites still OK |

Wave C implements until B2 is green **and** hardened DoD (parity / quit / tray) is met. Prior `406b59c` does **not** close Wave C. Never push.

---

## Commit / TDD order (PM-compatible)

| Order | Wave | WS     | Example commit subject                                                                                         |
| ----- | ---- | ------ | -------------------------------------------------------------------------------------------------------------- |
| 1     | A    | R-WS-1 | `chore: enforce LF via gitattributes and renormalize`                                                          |
| 2     | B    | R-WS-2 | `refactor: remove tray as top-level CLI mode`                                                                  |
| 3     | B    | R-WS-4 | `test: require real Croft/Loft startup contracts` (RED commit)                                                 |
| 4     | C    | R-WS-3 | `feat: wire FrankenTUI Croft TTY and Loft Web startup` *(landed; insufficient alone)*                          |
| 4b    | C    | R-WS-3 | `feat: Croft/Loft shared Model, q quit, Loft tray` *(required to re-close Wave C)*                             |
| 5     | D    | R-WS-5 | `docs: align README and plans with 0001 revise`                                                                |

Per agent: **failing tests → impl → green → commit** for R-WS-3; R-WS-4’s RED commit is intentionally failing until order 4/4b.

**FrankenTUI deps (Wave C):** crates.io **0.7.x** — `ftui`, `ftui-runtime`, `ftui-tty`, `ftui-web` (plus needed transitive `ftui-*`). Nightly if required. **Minimum shared UI:** bordered viewport + text that press `q` quits. Hello-tick without shared Model/App parity = **FAIL**.

---

## Smoke gate — revise

| Gate                                      | Minimum WS      | Command / check / FAIL                                                                                          |
| ----------------------------------------- | --------------- | --------------------------------------------------------------------------------------------------------------- |
| Tree is LF                                | R-WS-1          | `.gitattributes` present; new edits LF                                                                          |
| Help has croft/loft, not tray             | R-WS-2 + R-WS-4 | `cargo run -- --help` — must **not** list tray mode                                                             |
| Dry-run remains as seam only              | R-WS-2          | **must not** be the sole acceptance for UI DoD                                                                  |
| Shared Model/App parity                   | R-WS-3 + R-WS-4 | Same FrankenTUI Model/App; only name + TTY/Web differ; border + press-`q` text — static HTML-only loft = **FAIL** |
| `q` quit both backends                    | R-WS-3          | Croft: `q` exits; Loft: `q` stops Web host (exit 0) — Ctrl+C-only quit for loft = **FAIL**                      |
| Loft tray on Windows host                 | R-WS-3          | Default `mcload loft` shows tray + BG, quiet unless `--verbose` — no tray on Windows host = **FAIL**            |
| `--no-tray` path                          | R-WS-3 + R-WS-5 | Foreground web server; documented for WSL/headless CI                                                           |
| Docs match hardened DoD                   | R-WS-5          | README + scaffolding plan/DoD checkboxes; no “optional tray / Ctrl+C primary quit” hedges                       |
| Merge ask                                 | After MC YES    | Plans/reports must **not** solicit merge until MC confirms parity + quit + tray                                 |

---

## Compact notes for PM

1. **Schedule Wave A alone** — LF renormalize is an exclusive write lock on the tree.
2. **One agent for Wave B** (R-WS-2 → R-WS-4) — avoids `cli.rs` / `tests/cli_modes.rs` thrash; two commits preferred (tray gone green → RED startup).
3. **Do not start R-WS-3 until B2 RED is on the branch.**
4. **Wave C reopened:** prior FTUI wire is not enough. Deliver **parity + `q` quit + Loft tray MUST**. Never restore `mcload tray`; never require `loft --tray`.
5. **R-WS-5 is last** — only after Wave C re-closes; full prompt-history / final report belong here.
6. Untouched modules (`config`, `metadata`, `queues`, `plugins`) stay out of revise agents’ file lists.
7. Original [0001-workload-split.md](./0001-workload-split.md) stays as scaffolding history; this file owns REVISE scheduling.
8. Do **not** open epic 0002 / 0005 for tray/quit/parity — those are **0001** hard requirements.
9. After green revise against hardened DoD: wait for MC manual Croft + Loft confirm before any merge request.
