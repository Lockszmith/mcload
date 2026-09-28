# Epic 0001 — Initial Scaffolding

> **Status:** REVISE — Wave C **implemented** (parity + `q` quit + Loft tray code); **merge gated** on MC Windows tray / Croft+Loft YES  
> **Binary:** `mcload`  
> **Tagline:** "There can be only one!"  
> **Epic branch:** `epic/scaffolding`  
> **Repo state at original plan time:** essentially empty — `.git` (no commits on `main`), `.vscode/mcload.code-workspace` only.  
> **Scaffolding verifier (historical):** Waves A–E green; `cargo test` **25/25**; see [0001-test-green-log.md](./0001-test-green-log.md).  
> **REVISE verifier:** Waves A–B green; Wave C re-impl via `third_party/frankentui` submodule + `loft_host` (not static HTML); await MC YES; see [0001-test-green-log.md](./0001-test-green-log.md).  
> **REVISE schedule:** [0001-revise-workload-split.md](./0001-revise-workload-split.md) (waves A–D / R-WS-1…5).  
> **Merge gate:** MC manual confirm that Croft + Loft meet **parity / `q` quit / tray** DoD. Do **not** ask to merge until that YES. **Never push.**

---

## Goal

Stand up a **compilable, testable single-crate Rust binary** named `mcload` with:

1. Cargo project + `main` entry
2. CLI (clap) for help/version + launch modes: **Croft** (TTY UI) and **Loft** (Web UI) — **no** top-level `tray` mode; Loft owns `--no-tray` / `--verbose` as needed
3. **Parity UX:** Croft and Loft run the **same** FrankenTUI `Model`/`App` — only allowed differences are display name (`"Croft"` vs `"Loft"`) and render backend (TTY vs Web). Minimum view: border around the viewport plus text that pressing `q` quits. Dry-run-only is **not** merge acceptance.
4. **Quit:** pressing `q` in the UI exits the app for **both** backends. For Loft, `q` **MUST** stop the Web host / daemon process (process exits 0) — not only hide a window. Ctrl+C may remain as an extra interrupt; it is **not** the primary documented quit path.
5. **Tray under Loft (MUST on tray-capable OS):** On Windows (and any OS with tray support), `mcload loft` **MUST** show a tray icon and keep running in the background after launch, with **no** CLI stdout/stderr unless `--verbose`. On OS without tray support, or when passed `--no-tray`, run the web server in the foreground (Ctrl+C allowed to exit). WSL / headless CI **document** `--no-tray`; native Windows host **MUST** show the icon. Not a CLI mode; **not** `loft --tray` required.
6. YAML config persistence (user + per-project) with CLI overrides
7. Metadata persistence layer API with **concurrent access** (storage backend stubbed behind the API is OK)
8. Queue stubs: **Activity** (Pause/Resume/Abort), **Gathering** (fresh metadata), **Reckoning** (fresh→ready); plugin engagement/fingerprint/similarity markers as empty hooks
9. Dev-container (Rust)
10. README + MIT LICENSE
11. TDD-proven tests: binary starts, config round-trip, metadata concurrent-access, **real Croft/Loft parity + quit + startup contracts**
12. Cross-compile notes for Windows, Linux x86_64, macOS aarch64
13. **LF line endings** for the repo (`.gitattributes` + local git eol hygiene + normalize commit)

Process constraints for implementers:

- **Tests first** (fail → implement → pass) per step below
- **One commit per completed TDD step** (or per numbered DoD item if a step is atomic)
- **NEVER push** to remote
- Keep this plan file **updated** as steps complete (checkboxes / status)
- Follow REVISE wave order in [0001-revise-workload-split.md](./0001-revise-workload-split.md)

---

## Non-goals (this epic)

- Real filesystem profiling / chunk hashing / similarity algorithms
- Real merge-to-SoT workflows
- Real plugin implementations (fingerprint, identity, similarity engines)
- Full FrankenTUI UI polish beyond the shared minimal Model/App (border + quit hint) — polish → epic 0006
- Rich tray menus / OS notifications beyond the required tray icon + background keep-alive under Loft
- Activity/Gathering/Reckoning worker loops (types + markers only)
- Publishing to crates.io
- CI pipelines beyond what the README documents for local/dev-container use
- `git push` of any kind
- ~~Dry-run-only stub success as UI acceptance~~ (**invalidated** — `--dry-run` remains as a test seam only)
- ~~Top-level `mcload tray` CLI mode~~ (**removed** in REVISE)
- ~~Parking Loft tray / quit / parity in epic 0005~~ (**invalidated** — those are **0001 MUST**)

---

## Architecture sketch

Single binary crate `mcload` (no workspace split yet — revisit only if FrankenTUI deps force it).

```
┌──────────────────────────────────────────────────┐
│                    mcload CLI                    │
│     clap: --help | --version | croft | loft      │
│     loft: [--no-tray] [--verbose]                │
└─────────────────┬────────────────┬───────────────┘
                  │                │
                  ▼                ▼
             ┌─────────┐     ┌─────────────────────┐
             │  Croft  │     │        Loft         │
             │ same    │     │ same FrankenTUI     │
             │ Model/  │     │ Model/App (Web)     │
             │ App TTY │     │ + tray+BG MUST when │
             │         │     │ OS supports tray    │
             └────┬────┘     └──────────┬──────────┘
                  │                     │
                  └──────────┬──────────┘
                             ▼
                 ┌───────────────────────┐
                 │   App / Runtime core  │
                 │  config · metadata ·  │
                 │  queues · plugins stub│
                 └───────────────────────┘
```

### Modules (suggested `src/` layout)

| Module        | Responsibility                                                                                          |
| ------------- | ------------------------------------------------------------------------------------------------------- |
| `main.rs`     | Binary entry; parse CLI; dispatch mode                                                                  |
| `cli.rs`      | clap `Args` / subcommands / overrides — modes: **croft**, **loft** only; loft `--no-tray` / `--verbose` |
| `config.rs`   | User + project YAML load/save; merge with CLI                                                           |
| `metadata/`   | Concurrent store trait + in-memory (or sled/sqlite) stub                                                |
| `queues/`     | Activity, Gathering, Reckoning stubs + `Fresh`/`Ready` markers                                          |
| `plugins/`    | Trait stubs: engagement, fingerprint, similarity                                                        |
| `ui/shared`   | Shared FrankenTUI `Model`/`App` (border + quit hint); Croft/Loft differ only by name + backend          |
| `ui/croft.rs` | Croft launch — FrankenTUI TTY (`ftui` / `ftui-tty`); `q` exits process                                  |
| `ui/loft.rs`  | Loft launch — FrankenTUI Web (`ftui-web`); `q` stops host (exit 0); tray+BG MUST when OS supports        |
| `error.rs`    | Shared error type (`thiserror` or `anyhow` for app)                                                     |
| `lib.rs`      | Library surface for integration tests                                                                   |

**Removed in REVISE:** `src/tray.rs` and top-level `feature = "tray"` as a standalone launch path. Tray support lives under Loft (`ui/loft.rs` or a Loft-private helper), not as `mcload tray`. **`loft --tray` is not a required mode** — tray is default-on when the OS supports it; use `--no-tray` to force foreground.

Library crate + binary: expose `mcload` as both `lib` and `bin` so tests can import config/metadata without spawning always.

---

## File tree (target after REVISE)

```
mcload/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile            # rust:bookworm or nightly if ftui requires nightly
├── .gitattributes            # * text=auto eol=lf (+ binary exceptions)
├── .gitignore
├── .vscode/                  # EXISTS — leave workspace file
├── Cargo.toml                # ftui 0.7.x stack; no standalone tray feature/mode
├── LICENSE                   # MIT
├── README.md
├── rust-toolchain.toml       # pin nightly if FrankenTUI 0.7 requires it
├── docs/
│   └── plans/
│       ├── MASTER.md
│       ├── 0001-scaffolding.md          # this file
│       ├── 0001-workload-split.md       # historical Waves A–E
│       └── 0001-revise-workload-split.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs                # croft | loft only; loft --no-tray / --verbose
│   ├── config.rs
│   ├── error.rs
│   ├── metadata/
│   ├── queues/
│   ├── plugins/
│   └── ui/
│       ├── mod.rs
│       ├── shared (or equivalent)  # same Model/App for Croft + Loft
│       ├── croft.rs                # TTY backend; q quits
│       └── loft.rs                 # Web backend; q stops host; tray+BG MUST
└── tests/
    ├── cli_starts.rs
    ├── cli_modes.rs          # croft/loft only; no tray mode; loft --no-tray / --verbose
    ├── config_roundtrip.rs
    ├── metadata_concurrent.rs
    └── … startup / parity / quit contracts (no tray_stub.rs)
```

**Deleted / folded:** `src/tray.rs`, `tests/tray_stub.rs`, clap `Tray` / `LaunchMode::Tray`.

---

## Dependencies (crates) with rationale

| Crate                                                           | Rationale                                                                                                                                                                                                                                                                     |
| --------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `clap` (derive)                                                 | CLI help/version/subcommands/overrides                                                                                                                                                                                                                                        |
| `serde` + `serde_yaml`                                          | Config + metadata DTO serialization                                                                                                                                                                                                                                           |
| `thiserror` (+ optional `anyhow` in bin)                        | Typed library errors; ergonomic main                                                                                                                                                                                                                                          |
| `tokio` (rt-multi-thread, sync, fs) **or** sync + UI-edge async | Concurrent metadata; prefer tokio if Loft host needs async                                                                                                                                                                                                                    |
| `directories` or `dirs`                                         | XDG/AppData user config path resolution                                                                                                                                                                                                                                       |
| `tracing` + `tracing-subscriber`                                | Structured logs                                                                                                                                                                                                                                                               |
| `tempfile` (dev)                                                | Isolated config/metadata test dirs                                                                                                                                                                                                                                            |
| `assert_cmd` + `predicates` (dev)                               | Binary smoke tests                                                                                                                                                                                                                                                            |
| FrankenTUI **0.7.x from crates.io**                             | Prefer published stack: `ftui`, `ftui-runtime`, `ftui-tty`, `ftui-web` (plus any transitive `ftui-*` the 0.7.0 graph needs). Use **nightly** if ftui requires it. Croft = TTY via App/Program; Loft = Web via `ftui-web` host path. **Same** `Model`/`App` for both; minimum = bordered viewport + press-`q`-to-quit text. |
| Tray (**MUST** under Loft when OS supports)                     | Default-on tray icon + background keep-alive for Loft on Windows / tray-capable OS; silent unless `--verbose`; `--no-tray` forces foreground. **Not** a top-level CLI mode; **not** `loft --tray` required. No standalone `feature = "tray"` launch path.                                                                          |

**REVISE strategy for FrankenTUI (overrides prior stub-default):**  
Wire real crates.io `ftui` 0.7.x deps. Croft and Loft **MUST** share one FrankenTUI `Model`/`App` (name + backend only differ). Croft starts TTY UI (or returns a **clear actionable error** only if the environment truly cannot run a TTY UI). Loft starts Web UI hosting that same app; pressing `q` exits the process (Web host stops). `--dry-run` remains as a test seam; **dry-run-only success is not acceptance for merge**.

**FAIL (UI / host):**
- Static HTML-only Loft (no shared FrankenTUI `Model`/`App`) = **FAIL**
- Divergent Croft vs Loft UX beyond name + render backend = **FAIL**
- Ctrl+C-only quit for Loft (no `q` → process exit 0) = **FAIL**
- No tray icon on native Windows host for default `mcload loft` = **FAIL**
- Requiring `loft --tray` or a top-level `tray` mode = **FAIL**

**Metadata storage (API first):** unchanged — `MetadataStore: Send + Sync` with in-memory concurrent default for epic 0001.

---

## Historical TDD steps (scaffolding — DONE)

Original Waves A–E completed 2026-09-27. Kept for audit; **do not re-run as acceptance** for merge after REVISE intent change.

### Step 0 — Repo hygiene
- [x] Add `.gitignore`, `LICENSE` (MIT), stub `README.md` with title/tagline only
- [x] Confirm empty-ish tree; do not delete `.vscode/mcload.code-workspace`

### Step 1 — Cargo binary skeleton
- [x] **Fail / Impl / Pass / Commit:** clap `--help` / `--version` (landed)

### Step 2 — CLI modes (dispatch stubs) — **partially superseded**
- [x] Historical: subcommands `croft`, `loft`, `tray` with dry-run stubs
- [x] **REVISE:** drop `tray` mode; keep `croft` / `loft` as real startup paths

### Step 3 — Config YAML + CLI override
- [x] Done (unchanged by REVISE)

### Step 4 — Metadata concurrent store
- [x] Done (unchanged by REVISE)

### Step 5 — Queue stubs
- [x] Done (unchanged by REVISE)

### Step 6 — Plugin + UI + tray stubs — **superseded for UI/tray**
- [x] Historical: dry-run stubs for Croft/Loft/tray; `feature = "tray"` reserved
- [x] **REVISE:** real Croft/Loft FrankenTUI startup; remove standalone tray module/CLI

### Step 7 — Dev-container
- [x] Done (stable toolchain sufficient for ftui 0.7 — no nightly pin required)

### Step 8 — README completeness
- [x] Historical done; **REVISE** drop tray CLI + document real startup (R-WS-5 / this pass)

### Step 9 — Cross-compile notes
- [x] Done (docs only)

---

## REVISE TDD steps (fail → implement → pass)

Execute per [0001-revise-workload-split.md](./0001-revise-workload-split.md). Mark `[ ]` → `[x]` as done. **Commit after each green step** (RED commit intentional for R-WS-4). Never push.

### R-Step 1 — LF line endings (Wave A / R-WS-1)
- [x] Add `.gitattributes`: `* text=auto eol=lf` + sensible binary exceptions
- [x] Local repo only: `git config --local core.autocrlf false` and/or `core.eol lf`
- [x] `git add --renormalize .`; own commit on `epic/scaffolding`
- [x] Commit: `639245f` `chore: enforce LF via gitattributes and renormalize`

### R-Step 2 — Remove tray CLI everywhere (Wave B / R-WS-2)
- [x] **Fail then green:** help/tests must not list `tray` as a mode
- [x] Delete `src/tray.rs`; drop standalone `feature = "tray"` launch path; strip clap `Tray` / `LaunchMode::Tray` / dispatch
- [x] Commit: `593f42d` `refactor: remove tray as top-level CLI mode`

### R-Step 3 — RED real-startup contracts (Wave B / R-WS-4)
- [x] Delete `tests/tray_stub.rs`; rewrite `tests/cli_modes.rs` (croft/loft only)
- [x] Add RED tests requiring **real** Croft TTY / Loft Web startup contracts (not dry-run-only acceptance)
- [x] Commit RED intentionally: `c4b6e17` `test: require real Croft/Loft startup contracts`
- [x] Do **not** implement FrankenTUI until this RED commit is on the branch

### R-Step 4 — Real FrankenTUI parity + quit + Loft tray (Wave C / R-WS-3)
- [x] Wire FrankenTUI via `third_party/frankentui` git submodule path deps (`ftui` / `ftui-tty` / `ftui-web`); nightly pin matches submodule
- [x] Shared FrankenTUI `Model`/`App` for Croft + Loft (only name + TTY/Web backend differ)
- [x] Minimum view: bordered viewport + text that press `q` quits
- [x] `mcload croft`: `q` exits process
- [x] `mcload loft`: `q` **stops Web host / daemon** (process exit 0); Ctrl+C is interrupt only, not primary quit docs
- [x] Tray under Loft **MUST** on Windows / macOS: icon + background; quiet unless `--verbose` *(MC: confirm tray icon on native Windows host)*
- [x] `--no-tray` (and no-tray OS): foreground web server; document for WSL/headless CI
- [x] Native Windows host default `mcload loft` **MUST** show tray — **FAIL** if missing *(await MC Windows check)*
- [x] `--dry-run` remains as seam; suite proves non-dry-run via `MCLOAD_STARTUP_PROBE=1` + startup / quit contracts
- [x] Prior commit: `406b59c` — **superseded** by submodule + `loft_host` WebHost + tray-under-Loft
- [x] Follow-up: parity + quit + tray seams green in CI; Windows tray manual gate remains

### R-Step 5 — Docs alignment (Wave D / R-WS-5)
- [ ] README, this plan’s DoD checkboxes, report/logs, MASTER summary, prompt-history — **after** Wave C re-closes
- [ ] Document: `q` quit for both; Loft tray default-on; `--no-tray` for WSL/CI; `--verbose` for loft logs
- [ ] Commit subject example: `docs: align README and plans with 0001 revise` *(blocked until Wave C DoD met)*

### R-Step 6 — MC manual merge gate (human)
- [ ] MC manually confirms Croft: shared UX + `q` quit
- [ ] MC manually confirms Loft: shared UX + `q` stops host + tray on Windows (or `--no-tray` only where documented)
- [ ] Only after explicit MC **YES** may PM merge locally — still **never push** unless separately ordered
- [ ] Plans must **not** solicit merge until that YES

---

## Manual verification commands

### Paste-ready (Linux / WSL / devcontainer)

```bash
# From repo root (Linux / WSL / devcontainer)
cargo build
cargo run -- croft                 # TTY; press q to quit (process exits)
cargo run -- loft --no-tray        # foreground Web; press q in UI to stop host
                                   # (WSL/headless: --no-tray is the documented path)

# Non-blocking probes (CI seam — not the merge bar alone)
MCLOAD_STARTUP_PROBE=1 cargo run -- croft
MCLOAD_STARTUP_PROBE=1 cargo run -- loft --no-tray
```

### Host (PowerShell — Windows)

```powershell
# From repo root — native Windows MUST show tray for default loft
cargo test
cargo run -- --help
cargo run -- --version
cargo run -- croft                 # shared UX; press q to quit
cargo run -- loft                  # tray icon + BG; quiet unless --verbose; q stops host
cargo run -- loft --verbose        # same + stdout/stderr logs
cargo run -- loft --no-tray        # foreground only (escape hatch / CI parity)
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- croft
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- loft --no-tray
# Seam only — NOT merge acceptance alone:
cargo run -- croft --dry-run
cargo run -- loft --dry-run
```

### Cross-compile (documented; optional local)

Unchanged matrix notes in README (Win / Linux x86_64 / macOS aarch64). Scaffolding documents; does not require producing all three artifacts.

---

## Open questions / decisions

| #   | Topic                            | Status / proposal                                                                                                                                                          |
| --- | -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | FrankenTUI dependency strategy   | **LOCKED (REVISE):** crates.io **0.7.x** (`ftui`, `ftui-runtime`, `ftui-tty`, `ftui-web`); nightly if required                                                             |
| 2   | Async runtime                    | Open — tokio everywhere vs sync core + async at UI edges                                                                                                                   |
| 3   | Metadata backend for scaffolding | **LOCKED (scaffolding):** in-memory concurrent + optional file dump                                                                                                        |
| 4   | Config schema v0 fields          | Open — minimal `{ data_dir, log_level, ui: { default_mode } }`                                                                                                             |
| 5   | CLI shape                        | **LOCKED:** subcommands `mcload croft` / `mcload loft` only; **no** `mcload tray`; loft flags: `--no-tray`, `--verbose`                                                    |
| 6   | Dry-run / test seam              | **LOCKED:** `--dry-run` remains; **not** sole UI acceptance                                                                                                                |
| 7   | Tray                             | **LOCKED:** no top-level tray CLI; Loft **MUST** tray+BG on tray-capable OS (Windows host required); `--no-tray` for no-tray OS / WSL / CI                                 |
| 8   | Edition / MSRV / toolchain       | Open — pin nightly in `rust-toolchain.toml` if ftui 0.7 requires it                                                                                                        |
| 9   | Single crate vs workspace        | Prefer single crate; revisit only if ftui graph forces it                                                                                                                  |
| 10  | Croft/Loft UX parity             | **LOCKED:** same FrankenTUI `Model`/`App`; only display name + TTY vs Web differ; min = border + press-`q` text                                                            |
| 11  | Quit path                        | **LOCKED:** `q` exits both; Loft `q` stops Web host (exit 0); Ctrl+C is extra interrupt only                                                                               |
| 12  | Merge                            | **LOCKED process:** await MC manual Croft+Loft confirm against hardened DoD; do not ask merge until YES; never push                                                        |

---

## Definition of Done

### Scaffolding (historical — met 2026-09-27; insufficient alone after REVISE)

- [x] `cargo test` passed on host with default features (25/25 at green log)
- [x] `cargo run -- --help` and `--version` work
- [x] YAML config + metadata concurrent + queue/plugin stubs + dev-container + MIT/README + cross notes
- [x] Commits per step; **no push**

### REVISE (required before merge consideration)

- [x] LF: `.gitattributes` + local eol config applied; normalize commit on `epic/scaffolding`
- [x] No top-level `tray` in clap/help/tests/docs; `src/tray.rs` / standalone tray feature launch path gone
- [x] **Parity:** Croft and Loft run the **same** FrankenTUI `Model`/`App` (only `"Croft"`/`"Loft"` name + TTY/Web backend differ); minimum bordered viewport + press-`q`-to-quit text
- [x] **Quit:** `q` exits Croft process; `q` stops Loft Web host / daemon (process exit 0). Ctrl+C is not the primary documented quit path
- [x] **Tray:** default `mcload loft` on Windows / macOS shows tray icon + stays in background; quiet unless `--verbose`; `--no-tray` / no-tray OS → foreground web server *(MC Windows icon check pending)*
- [x] Startup / parity / quit contract tests green; dry-run-only is **not** the merge bar
- [x] README + plans + prompt-history align with parity, `q` quit, and Loft tray MUST *(Wave D — largely done; final after MC YES)*
- [ ] MC manually confirms Croft + Loft against this DoD → then (and only then) merge may be requested
- [x] **Never push**

### Explicit FAIL criteria (Wave C / merge)

| FAIL if…                                              | Why                                                          |
| ----------------------------------------------------- | ------------------------------------------------------------ |
| Static HTML-only Loft (no shared FrankenTUI Model/App) | No UX parity with Croft                                      |
| Croft/Loft UX differ beyond name + render backend      | Violates same-Model/App rule                                 |
| Ctrl+C-only quit for Loft                              | `q` MUST stop the Web host (exit 0)                          |
| `q` only hides a window / does not exit Loft process   | Daemon must terminate                                        |
| No tray icon on native Windows for default `loft`      | Tray under Loft is MUST on Windows host                      |
| `loft --tray` required, or top-level `tray` mode       | Tray is default-on under Loft; not a CLI mode                |
| Tray / quit / parity deferred to epic 0005             | Hardened 0001 DoD owns these                                 |
| Dry-run-only treated as UI acceptance                  | Seam only                                                    |

---

## Agent notes (for implementers)

- Prefer small diffs; do not invent dedup algorithms in this epic.
- Keep Croft/Loft behind a thin launch surface sharing one Model/App; polish beyond border+quit stays epic 0006.
- Tray icon + BG under Loft is **0001 MUST** on tray-capable OS — do not park in 0005.
- Schedule code work via [0001-revise-workload-split.md](./0001-revise-workload-split.md); historical [0001-workload-split.md](./0001-workload-split.md) is audit-only for Waves A–E.
- Update `docs/plans/MASTER.md` when REVISE status or merge gate changes.
- Collapsible README sections use HTML `<details><summary>…</summary>…</details>`.
