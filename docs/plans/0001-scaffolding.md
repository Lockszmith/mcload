# Epic 0001 — Initial Scaffolding

> **Status:** REVISE — impl green, awaiting MC Croft/Loft confirm  
> **Binary:** `mcload`  
> **Tagline:** "There can be only one!"  
> **Epic branch:** `epic/scaffolding`  
> **Repo state at original plan time:** essentially empty — `.git` (no commits on `main`), `.vscode/mcload.code-workspace` only.  
> **Scaffolding verifier (historical):** Waves A–E green; `cargo test` **25/25**; see [0001-test-green-log.md](./0001-test-green-log.md).  
> **REVISE verifier:** Waves A–C green; Wave D (docs) in progress / this pass; `cargo test` **30/30**; see GREEN baseline in [0001-test-green-log.md](./0001-test-green-log.md).  
> **REVISE schedule:** [0001-revise-workload-split.md](./0001-revise-workload-split.md) (waves A–D / R-WS-1…5).  
> **Merge gate:** MC manual confirm that Croft + Loft **actually start**. Do **not** ask to merge until that YES. **Never push.**

---

## Goal

Stand up a **compilable, testable single-crate Rust binary** named `mcload` with:

1. Cargo project + `main` entry
2. CLI (clap) for help/version + launch modes: **Croft** (TTY UI) and **Loft** (Web UI) — **no** top-level `tray` mode
3. **Real FrankenTUI startup** for Croft + Loft (minimal hello-tick UI is enough; dry-run-only is **not** merge acceptance)
4. Tray capability (when OS supports it) owned **under Loft** as background + tray — not a separate CLI mode
5. YAML config persistence (user + per-project) with CLI overrides
6. Metadata persistence layer API with **concurrent access** (storage backend may be stubbed behind the API)
7. Queue stubs: **Activity** (Pause/Resume/Abort), **Gathering** (fresh metadata), **Reckoning** (fresh→ready); plugin engagement/fingerprint/similarity markers as empty hooks
8. Dev-container (Rust)
9. README + MIT LICENSE
10. TDD-proven tests: binary starts, config round-trip, metadata concurrent-access, **real Croft/Loft startup contracts**
11. Cross-compile notes for Windows, Linux x86_64, macOS aarch64
12. **LF line endings** for the repo (`.gitattributes` + local git eol hygiene + normalize commit)

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
- Full FrankenTUI UI polish / production Loft WASM host (startup + minimal tick UI only; polish → epic 0006)
- Production tray menus / OS notifications beyond “Loft may hold a tray when the OS supports it”
- Activity/Gathering/Reckoning worker loops (types + markers only)
- Publishing to crates.io
- CI pipelines beyond what the README documents for local/dev-container use
- `git push` of any kind
- ~~Dry-run-only stub success as UI acceptance~~ (**invalidated** — `--dry-run` may remain as a test seam only)
- ~~Top-level `mcload tray` CLI mode~~ (**removed** in REVISE)

---

## Architecture sketch

Single binary crate `mcload` (no workspace split yet — revisit only if FrankenTUI deps force it).

```
┌──────────────────────────────────────────────────┐
│                    mcload CLI                    │
│     clap: --help | --version | croft | loft      │
└─────────────────┬────────────────┬───────────────┘
                  │                │
                  ▼                ▼
             ┌─────────┐     ┌─────────────────────┐
             │  Croft  │     │        Loft         │
             │ Franken │     │ FrankenTUI Web UI   │
             │ TUI TTY │     │ (+ optional tray /  │
             │         │     │  BG when OS allows) │
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

| Module        | Responsibility                                                            |
| ------------- | ------------------------------------------------------------------------- |
| `main.rs`     | Binary entry; parse CLI; dispatch mode                                    |
| `cli.rs`      | clap `Args` / subcommands / overrides — modes: **croft**, **loft** only   |
| `config.rs`   | User + project YAML load/save; merge with CLI                             |
| `metadata/`   | Concurrent store trait + in-memory (or sled/sqlite) stub                  |
| `queues/`     | Activity, Gathering, Reckoning stubs + `Fresh`/`Ready` markers            |
| `plugins/`    | Trait stubs: engagement, fingerprint, similarity                          |
| `ui/croft.rs` | Croft launch — **real** FrankenTUI TTY (`ftui` / `ftui-tty` App/Program)  |
| `ui/loft.rs`  | Loft launch — **real** FrankenTUI Web (`ftui-web` host); optional tray/BG |
| `error.rs`    | Shared error type (`thiserror` or `anyhow` for app)                       |
| `lib.rs`      | Library surface for integration tests                                     |

**Removed in REVISE:** `src/tray.rs` and top-level `feature = "tray"` as a standalone launch path. Any tray support lives under Loft (`ui/loft.rs` or a Loft-private helper), not as `mcload tray`.

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
│   ├── cli.rs                # croft | loft only
│   ├── config.rs
│   ├── error.rs
│   ├── metadata/
│   ├── queues/
│   ├── plugins/
│   └── ui/
│       ├── mod.rs
│       ├── croft.rs          # real TTY startup
│       └── loft.rs           # real Web startup (+ optional tray under Loft)
└── tests/
    ├── cli_starts.rs
    ├── cli_modes.rs          # croft/loft only; no tray
    ├── config_roundtrip.rs
    ├── metadata_concurrent.rs
    └── … startup contracts (no tray_stub.rs)
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
| FrankenTUI **0.7.x from crates.io**                             | Prefer published stack: `ftui`, `ftui-runtime`, `ftui-tty`, `ftui-web` (plus any transitive `ftui-*` the 0.7.0 graph needs). Use **nightly** if ftui requires it. Croft = TTY via App/Program; Loft = Web via `ftui-web` host path. Minimal hello-tick UI satisfies “starts”. |
| Tray (optional, under Loft only)                                | Only if OS supports it and Loft chooses to background; **not** a top-level CLI mode or required `feature = "tray"` launch path                                                                                                                                                |

**REVISE strategy for FrankenTUI (overrides prior stub-default):**  
Wire real crates.io `ftui` 0.7.x deps. Croft must start TTY UI (or return a **clear actionable error** only if the environment truly cannot run a TTY UI). Loft must start Web UI. `--dry-run` may remain as a test seam; **dry-run-only success is not acceptance for merge**.

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

### R-Step 4 — Real FrankenTUI startup (Wave C / R-WS-3)
- [x] Wire crates.io `ftui` / `ftui-tty` / `ftui-web` (0.7.x); stable toolchain OK
- [x] `mcload croft` starts FrankenTUI TTY UI (or clear `TtyUnavailable` if env cannot)
- [x] `mcload loft` starts FrankenTUI Web UI and prints local listen URL; tray under Loft not required yet
- [x] `--dry-run` remains as seam; suite proves non-dry-run via `MCLOAD_STARTUP_PROBE=1` + startup contracts
- [x] Commit: `406b59c` `feat: wire FrankenTUI Croft TTY and Loft Web startup`

### R-Step 5 — Docs alignment (Wave D / R-WS-5)
- [x] README, this plan’s DoD checkboxes, report/logs, MASTER summary, prompt-history REVISE section (this pass; parent commits)
- [ ] Commit subject example: `docs: align README and plans with 0001 revise` *(parent commits)*

### R-Step 6 — MC manual merge gate (human)
- [ ] MC manually confirms Croft starts
- [ ] MC manually confirms Loft starts
- [ ] Only after explicit MC **YES** may PM merge locally — still **never push** unless separately ordered
- [ ] Plans must **not** solicit merge until that YES

---

## Manual verification commands

### Paste-ready (Linux / WSL / devcontainer)

```bash
# From repo root (Linux / WSL / devcontainer)
cargo build
cargo run -- croft          # needs interactive TTY; press q to quit
cargo run -- loft           # note printed URL; open in browser; Ctrl+C to stop

# Non-blocking probes (CI seam — not the merge bar alone)
MCLOAD_STARTUP_PROBE=1 cargo run -- croft
MCLOAD_STARTUP_PROBE=1 cargo run -- loft
```

### Host (PowerShell — Windows)

```powershell
# From repo root
cargo test
cargo run -- --help
cargo run -- --version
cargo run -- croft          # must start TTY UI (or clear TtyUnavailable)
cargo run -- loft           # must print http://127.0.0.1:PORT and serve HTML
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- croft
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- loft
# Optional seam only — NOT merge acceptance alone:
cargo run -- croft --dry-run
cargo run -- loft --dry-run
```

### Cross-compile (documented; optional local)

Unchanged matrix notes in README (Win / Linux x86_64 / macOS aarch64). Scaffolding documents; does not require producing all three artifacts.

---

## Open questions / decisions

| #   | Topic                            | Status / proposal                                                                                                                                                                       |
| --- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | FrankenTUI dependency strategy   | **LOCKED (REVISE):** crates.io **0.7.x** (`ftui`, `ftui-runtime`, `ftui-tty`, `ftui-web`); nightly if required                                                                          |
| 2   | Async runtime                    | Open — tokio everywhere vs sync core + async at UI edges                                                                                                                                |
| 3   | Metadata backend for scaffolding | **LOCKED (scaffolding):** in-memory concurrent + optional file dump                                                                                                                     |
| 4   | Config schema v0 fields          | Open — minimal `{ data_dir, log_level, ui: { default_mode } }`                                                                                                                          |
| 5   | CLI shape                        | **LOCKED:** subcommands `mcload croft` / `mcload loft` only; **no** `mcload tray`                                                                                                       |
| 6   | Dry-run / test seam              | **LOCKED:** `--dry-run` may remain; **not** sole UI acceptance                                                                                                                          |
| 7   | Tray                             | **LOCKED:** no top-level tray CLI; optional capability **under Loft** when OS supports it                                                                                               |
| 8   | Edition / MSRV / toolchain       | Open — pin nightly in `rust-toolchain.toml` if ftui 0.7 requires it                                                                                                                     |
| 9   | Single crate vs workspace        | Prefer single crate; revisit only if ftui graph forces it                                                                                                                               |
| 10  | Loft tray test seam              | **PM-lockable:** prefer **no** new top-level CLI arg; if a flag is needed for tests only, keep it optional under `loft` and document — default is Loft-owned auto when OS supports tray |
| 11  | Merge                            | **LOCKED process:** await MC manual Croft+Loft confirm; do not ask merge until YES; never push                                                                                          |

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
- [x] `mcload croft` starts FrankenTUI TTY UI (or clear actionable error only if env cannot)
- [x] `mcload loft` starts FrankenTUI Web UI; tray (if any) is Loft-owned optional capability
- [x] Startup contract tests green; dry-run-only is **not** the merge bar
- [x] README + plans + prompt-history align with Croft+Loft-only CLI and real startup *(Wave D / this pass)*
- [ ] MC manually confirms Croft + Loft start → then (and only then) merge may be requested
- [x] **Never push**

---

## Agent notes (for implementers)

- Prefer small diffs; do not invent dedup algorithms in this epic.
- Keep Croft/Loft behind a thin launch surface; polish stays epic 0006.
- Schedule code work via [0001-revise-workload-split.md](./0001-revise-workload-split.md); historical [0001-workload-split.md](./0001-workload-split.md) is audit-only for Waves A–E.
- Update `docs/plans/MASTER.md` when REVISE status or merge gate changes.
- Collapsible README sections use HTML `<details><summary>…</summary>…</details>`.
