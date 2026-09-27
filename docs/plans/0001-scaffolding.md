# Epic 0001 — Initial Scaffolding

> **Status:** PLANNED (not implemented)  
> **Binary:** `mcload`  
> **Tagline:** "There can be only one!"  
> **Repo state at plan time:** essentially empty — `.git` (no commits on `main`), `.vscode/mcload.code-workspace` only. No `Cargo.toml`, `src/`, `README`, or `LICENSE` yet.

---

## Goal

Stand up a **compilable, testable single-crate Rust binary** named `mcload` with:

1. Cargo project + `main` entry
2. CLI (clap) for help/version + launch modes: **Croft** (TUI), **Loft** (WebUI), **tray/background**
3. FrankenTUI hooks for Croft + Loft per [docs.frankentui.com](https://docs.frankentui.com) (stubs OK; compile-gated if full stack unavailable)
4. System tray icon stub
5. YAML config persistence (user + per-project) with CLI overrides
6. Metadata persistence layer API with **concurrent access** (storage backend may be stubbed behind the API)
7. Queue stubs: **Activity** (Pause/Resume/Abort), **Gathering** (fresh metadata), **Reckoning** (fresh→ready); plugin engagement/fingerprint/similarity markers as empty hooks
8. Dev-container (Rust)
9. README + MIT LICENSE
10. Minimal TDD-proven tests: binary starts, config round-trip, metadata concurrent-access contracts
11. Cross-compile notes for Windows, Linux x86_64, macOS aarch64

Process constraints for implementers:

- **Tests first** (fail → implement → pass) per step below
- **One commit per completed TDD step** (or per numbered DoD item if a step is atomic)
- **NEVER push** to remote
- Keep this plan file **updated** as steps complete (checkboxes / status)

---

## Non-goals (this epic)

- Real filesystem profiling / chunk hashing / similarity algorithms
- Real merge-to-SoT workflows
- Real plugin implementations (fingerprint, identity, similarity engines)
- Full FrankenTUI UI polish or production Loft WASM host
- Production tray menus / OS notifications beyond a stub
- Activity/Gathering/Reckoning worker loops (types + markers only)
- Publishing to crates.io
- CI pipelines beyond what the README documents for local/dev-container use
- `git push` of any kind

---

## Architecture sketch

Single binary crate `mcload` (no workspace split yet — revisit if FrankenTUI path deps force a workspace).

```
┌─────────────────────────────────────────────────────────────┐
│                         mcload CLI                          │
│  clap: --help | --version | croft | loft | tray | …flags    │
└────────────┬────────────────┬────────────────┬──────────────┘
             │                │                │
             ▼                ▼                ▼
        ┌─────────┐     ┌─────────┐     ┌────────────┐
        │  Croft  │     │  Loft   │     │ Tray/BG    │
        │ Franken │     │ Franken │     │ stub tray  │
        │ TUI TTY │     │ Web/WASM│     │ + idle loop│
        └────┬────┘     └────┬────┘     └─────┬──────┘
             │                │                │
             └────────────┬───┴────────────────┘
                          ▼
              ┌───────────────────────┐
              │   App / Runtime core  │
              │  config · metadata ·  │
              │  queues · plugins stub│
              └───────────────────────┘
```

### Modules (suggested `src/` layout)

| Module        | Responsibility                                                 |
| ------------- | -------------------------------------------------------------- |
| `main.rs`     | Binary entry; parse CLI; dispatch mode                         |
| `cli.rs`      | clap `Args` / subcommands / overrides                          |
| `config.rs`   | User + project YAML load/save; merge with CLI                  |
| `metadata/`   | Concurrent store trait + in-memory (or sled/sqlite) stub       |
| `queues/`     | Activity, Gathering, Reckoning stubs + `Fresh`/`Ready` markers |
| `plugins/`    | Trait stubs: engagement, fingerprint, similarity               |
| `ui/croft.rs` | Croft launch hook (FrankenTUI TTY)                             |
| `ui/loft.rs`  | Loft launch hook (FrankenTUI web/WASM host stub)               |
| `tray.rs`     | System tray stub (feature-gated)                               |
| `error.rs`    | Shared error type (`thiserror` or `anyhow` for app)            |
| `lib.rs`      | Library surface for integration tests                          |

Library crate + binary: expose `mcload` as both `lib` and `bin` so tests can import config/metadata without spawning always.

---

## File tree to create

```
mcload/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile            # rust:bookworm or rustlang/rust:nightly if ftui requires nightly
├── .gitignore                # /target, *.pdb, .env, OS junk, local config overrides
├── .vscode/                  # EXISTS — leave workspace file; optional rust-analyzer settings later
├── Cargo.toml
├── LICENSE                   # MIT
├── README.md
├── rust-toolchain.toml       # pin stable (or nightly if FrankenTUI forces it — see Open Questions)
├── docs/
│   └── plans/
│       ├── MASTER.md
│       └── 0001-scaffolding.md   # this file
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── error.rs
│   ├── tray.rs
│   ├── metadata/
│   │   ├── mod.rs
│   │   ├── store.rs          # MetadataStore trait + concurrent impl
│   │   └── types.rs          # Snapshot record stubs
│   ├── queues/
│   │   ├── mod.rs
│   │   ├── activity.rs       # Pause/Resume/Abort
│   │   ├── gathering.rs      # fresh metadata intake
│   │   └── reckoning.rs      # fresh → ready
│   ├── plugins/
│   │   ├── mod.rs
│   │   └── stubs.rs          # engagement / fingerprint / similarity markers
│   └── ui/
│       ├── mod.rs
│       ├── croft.rs
│       └── loft.rs
└── tests/
    ├── cli_starts.rs         # --help / --version exit 0
    ├── config_roundtrip.rs
    └── metadata_concurrent.rs
```

Optional later (not required this epic): `examples/`, `benches/`, `scripts/cross-build.ps1`.

---

## Dependencies (crates) with rationale

| Crate                                                                                    | Rationale                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `clap` (derive)                                                                          | CLI help/version/subcommands/overrides; industry default                                                                                                                                                        |
| `serde` + `serde_yaml`                                                                   | Config + metadata DTO serialization                                                                                                                                                                             |
| `thiserror` (+ optional `anyhow` in bin)                                                 | Typed library errors; ergonomic main                                                                                                                                                                            |
| `tokio` (rt-multi-thread, sync, fs) **or** `parking_lot` + sync API                      | Concurrent metadata access; prefer tokio if Loft/tray need async later — **lock in Open Questions**                                                                                                             |
| `directories` or `dirs`                                                                  | XDG/AppData user config path resolution                                                                                                                                                                         |
| `tracing` + `tracing-subscriber`                                                         | Structured logs for stub modes                                                                                                                                                                                  |
| `tempfile` (dev)                                                                         | Isolated config/metadata test dirs                                                                                                                                                                              |
| `assert_cmd` + `predicates` (dev)                                                        | Binary smoke tests (`--help`, `--version`)                                                                                                                                                                      |
| Tray: `tray-icon` + `winit` **or** `ksni` (Linux)                                        | Stub only; **feature `tray`** so headless CI/container builds skip native tray                                                                                                                                  |
| FrankenTUI: `ftui-core`, and when available `ftui-runtime` / `ftui-widgets` / `ftui-web` | Croft = TTY path; Loft = web backend hooks. **Only `ftui-core`, `ftui-layout`, `ftui-i18n` are reliably on crates.io today**; full stack may need git/path dep or compile stubs behind `feature = "frankentui"` |

**Scaffolding strategy for FrankenTUI:**  
Follow basics from [docs.frankentui.com](https://docs.frankentui.com) (Croft = TTY / Loft = web). Ship `ui/croft.rs` / `ui/loft.rs` that compile with **stubs by default**. Gate real ftui imports behind `feature = "frankentui"` so `cargo test` / `cargo build` succeed without cloning FrankenTUI. Document how to enable the feature once dependency strategy is locked.

**Metadata storage (API first):**  
Define `MetadataStore: Send + Sync` with `get` / `put` / `list` (names TBD). Default impl: `Arc<RwLock<…>>` in-memory, or `sled` / `rusqlite` with connection pooling if persistence must survive process exit in scaffolding tests. Prefer **in-memory + file snapshot YAML/JSON** for epic 0001 unless concurrent file DB is trivial.

---

## TDD steps (fail → implement → pass)

Mark each step `[ ]` → `[x]` as done. **Commit after each green step.** Never push.

### Step 0 — Repo hygiene
- [ ] Add `.gitignore`, `LICENSE` (MIT), stub `README.md` with title/tagline only
- [ ] Confirm empty-ish tree; do not delete `.vscode/mcload.code-workspace`

### Step 1 — Cargo binary skeleton
- [ ] **Fail:** `tests/cli_starts.rs` expects `cargo run -- --version` / `--help` (or `assert_cmd`) → no binary yet
- [ ] **Impl:** `cargo init --name mcload`, `lib.rs` + `main.rs`, clap `--help` / `--version`
- [ ] **Pass:** binary prints version derived from `CARGO_PKG_VERSION`; help lists modes
- [ ] Commit: `chore: init mcload binary with clap help/version`

### Step 2 — CLI modes (dispatch stubs)
- [ ] **Fail:** tests assert subcommands/flags `croft`, `loft`, `tray` exist in `--help`
- [ ] **Impl:** clap subcommands; each mode calls stub that returns `Ok(())` quickly (no blocking UI in tests)
- [ ] **Pass:** `mcload croft --dry-run` (or env `MCLOAD_UI=stub`) exits 0 without opening TUI
- [ ] Commit: `feat: add croft/loft/tray CLI mode stubs`

### Step 3 — Config YAML + CLI override
- [ ] **Fail:** `tests/config_roundtrip.rs` — load missing file creates defaults; save; reload; CLI override wins over file
- [ ] **Impl:** `config.rs` — user config path via `directories`; optional `--project` / `--config`; merge order: defaults < user YAML < project YAML < CLI
- [ ] **Pass:** round-trip + override tests green
- [ ] Commit: `feat: yaml config load/save with CLI overrides`

### Step 4 — Metadata concurrent store
- [ ] **Fail:** `tests/metadata_concurrent.rs` — N threads/tasks read/write same store without panic/data race; contract: last-write-wins or documented merge
- [ ] **Impl:** `MetadataStore` trait + `ConcurrentMemoryStore` (`Arc<RwLock<_>>` or tokio `RwLock`)
- [ ] **Pass:** concurrent stress test (e.g. 8 threads × 100 ops) green under `--test-threads=1` and default
- [ ] Commit: `feat: concurrent metadata store stub`

### Step 5 — Queue stubs
- [ ] **Fail:** unit tests in `queues/*` — Activity Pause/Resume/Abort transitions; Gathering items marked `Fresh`; Reckoning promotes `Fresh` → `Ready`
- [ ] **Impl:** enums/structs only; no workers
- [ ] **Pass:** transition tests green
- [ ] Commit: `feat: activity/gathering/reckoning queue stubs`

### Step 6 — Plugin + UI + tray stubs
- [ ] **Fail:** compile-time / unit tests that plugin traits and `croft`/`loft`/`tray` entrypoints exist and return stub results
- [ ] **Impl:** `plugins/stubs.rs`, `ui/croft.rs`, `ui/loft.rs`, `tray.rs` (`#[cfg(feature = "tray")]`)
- [ ] **Pass:** default features build; optional `tray` feature builds on host OS when deps available
- [ ] Commit: `feat: ui/tray/plugin stubs for croft and loft`

### Step 7 — Dev-container
- [ ] **Fail:** N/A (infra); verify Dockerfile builds
- [ ] **Impl:** `.devcontainer/` with Rust toolchain matching `rust-toolchain.toml`; pre-install `pkg-config` etc. as needed
- [ ] **Pass:** `cargo test` inside container (document command)
- [ ] Commit: `chore: add rust devcontainer`

### Step 8 — README completeness
- [ ] Purpose, tagline, usage (CLI modes)
- [ ] Collapsible `<details>` sections: **Dev Container**, **Tests**, **Multi-platform builds** (Win / Linux x86_64 / macOS aarch64)
- [ ] Commit: `docs: expand README with usage and build notes`

### Step 9 — Cross-compile notes (docs only + smoke if feasible)
- [ ] Document targets and toolchains in README (no requirement to produce all three artifacts in CI this epic)
- [ ] Optional smoke: `cargo check` for host target only required
- [ ] Commit if anything beyond README changed

---

## Manual verification commands

### Host (PowerShell — Windows)

```powershell
# From repo root
cargo test
cargo run -- --help
cargo run -- --version
cargo run -- croft --dry-run   # or whatever dry-run flag is chosen
cargo run -- loft --dry-run
cargo check

# Config smoke (after Step 3)
$env:MCLOAD_CONFIG_DIR = Join-Path $env:TEMP "mcload-test-config"
cargo test --test config_roundtrip -- --nocapture

# Concurrent metadata
cargo test --test metadata_concurrent -- --nocapture
```

### Dev Container

```bash
# VS Code / Cursor: "Reopen in Container", then:
cargo test
cargo run -- --help
```

Or CLI (if `devcontainer` CLI installed):

```powershell
devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . cargo test
```

### Cross-compile (documented; optional local)

```powershell
# Linux x86_64 from Windows (example — requires linker/toolchain setup)
rustup target add x86_64-unknown-linux-gnu
# cargo build --release --target x86_64-unknown-linux-gnu

# macOS aarch64 (typically on Apple Silicon host or with osxcross — document limits)
rustup target add aarch64-apple-darwin

# Windows MSVC host
cargo build --release --target x86_64-pc-windows-msvc
```

Note in README: cross-OS binaries often need `cross` (https://github.com/cross-rs/cross) or native hosts; scaffolding only **documents** the matrix.

---

## Open questions / decisions to lock

1. **FrankenTUI dependency strategy:** crates.io-only stubs vs git submodule/path to full FrankenTUI (nightly?). Default proposal: **feature `frankentui` off**; stubs on by default.
2. **Async runtime:** tokio everywhere vs sync core + async only at UI edges?
3. **Metadata backend for scaffolding:** pure memory + optional file dump vs `sled`/`sqlite` from day one?
4. **Config schema v0 fields:** minimal `{ data_dir, log_level, ui: { default_mode } }` — exact keys TBD.
5. **CLI shape:** subcommands (`mcload croft`) vs flags (`mcload --croft`)? Proposal: **subcommands**.
6. **Dry-run / test seam for UI:** `--dry-run` flag vs `MCLOAD_UI=stub` env — pick one primary.
7. **Tray crate choice** and whether tray is default-off (`feature = "tray"`).
8. **Edition / MSRV:** 2021 vs 2024; pin in `rust-toolchain.toml` and README.
9. **Single crate vs workspace** if FrankenTUI path deps appear.

---

## Definition of Done

- [ ] `cargo test` passes on host with **default features**
- [ ] `cargo run -- --help` and `--version` work
- [ ] Modes `croft`, `loft`, `tray` exist as stubs (tray may be feature-gated)
- [ ] YAML config load/save + CLI override covered by tests
- [ ] Metadata store concurrent-access contract covered by tests
- [ ] Activity / Gathering / Reckoning stubs with Fresh/Ready (and Pause/Resume/Abort) exist
- [ ] Plugin stub module present (no real engines)
- [ ] `.devcontainer` builds and can run `cargo test`
- [ ] `LICENSE` is MIT; `README.md` has purpose, usage, and collapsible Dev Container / Tests / Cross-build sections
- [ ] Cross-compile matrix documented for Win, Linux x86_64, macOS aarch64
- [ ] This plan’s checkboxes updated; commits exist per step; **no push**
- [ ] `MASTER.md` still lists 0001 as current epic until explicitly closed

---

## Agent notes (for implementers)

- Prefer small diffs; do not invent dedup algorithms in this epic.
- If FrankenTUI APIs churn, keep Croft/Loft behind a thin `UiBackend` trait.
- Update `docs/plans/MASTER.md` status when this epic moves In Progress → Done.
- Collapsible README sections use HTML `<details><summary>…</summary>…</details>` for GitHub/Cursor rendering.
