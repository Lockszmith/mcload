# McLoad

> There can be only one!

**McLoad** is a single-binary Rust tool for filesystem load / metadata workflows with one Source of Truth in mind. It exposes two launch modes — **Croft** (FrankenTUI TTY UI) and **Loft** (FrankenTUI Web UI) — that share the **same** FrankenTUI `Model`/`App` (only the display name and TTY vs Web backend differ), plus YAML config, a concurrent metadata store API, and queue stubs (Activity / Gathering / Reckoning). On tray-capable OS (including native Windows), **Loft** shows a tray icon and stays in the background after launch — not a separate CLI mode.

Licensed under the [MIT License](LICENSE).

## Usage

Build and run from the repo root (Rust nightly pin — see `rust-toolchain.toml`; matches `third_party/frankentui`):

```bash
git submodule update --init --recursive
cargo build
cargo run -- --help
cargo run -- --version
```

| Mode / flag                | Purpose                                                                                |
| -------------------------- | -------------------------------------------------------------------------------------- |
| `mcload --help`            | Print CLI help and exit                                                                |
| `mcload --version`         | Print version (`CARGO_PKG_VERSION`) and exit                                           |
| `mcload croft`             | Croft — FrankenTUI TTY UI (needs a real TTY; press `q` to quit)                        |
| `mcload loft`              | Loft — FrankenTUI Web UI; tray+BG on tray-capable OS; opens browser; press `q` to stop |
| `mcload loft --no-tray`    | Loft foreground web server (WSL / headless CI; OS without tray); press `q` to stop     |
| `mcload loft --no-browser` | Do not open the default browser on launch (tray still has **Open in Browser**)         |
| `mcload loft --verbose`    | Loft with extra stdout/stderr logs                                                     |
| `… --dry-run`              | Test seam only: exit 0 without opening UI (not merge / UI acceptance)                  |
| `--config <path>`          | Override config YAML path (global)                                                     |
| `--project <dir>`          | Per-project config directory (global)                                                  |
| `--log-level <level>`      | Override log level from CLI (global)                                                   |

**Croft** requires an interactive TTY. Without one it exits with a clear `TtyUnavailable` error (re-run in a real terminal, or use `--dry-run` / `MCLOAD_STARTUP_PROBE=1`). Press **`q`** to quit. **Loft** hosts the same FrankenTUI app over Web: press **`q`** in the UI to stop the Web host (process exits 0). On launch, Loft opens the default browser unless `--no-browser`; when started from a terminal it prints the listen URL. On native Windows (and macOS), default `mcload loft` shows a tray icon (menu: **Open in Browser**, **Quit**) and runs in the background. Use `--no-tray` on WSL / headless CI or when the OS has no tray — then the web server runs in the foreground. Ctrl+C is an interrupt escape hatch only, not the primary quit path.

FrankenTUI is vendored as a git submodule at [`third_party/frankentui`](third_party/frankentui) (path dependencies). After clone: `git submodule update --init --recursive`.

### Paste-ready startup (Croft + Loft)

```bash
# From repo root (Linux / WSL / devcontainer)
cargo build
cargo run -- croft                 # needs interactive TTY; press q to quit
cargo run -- loft --no-tray        # WSL/CI: foreground Web; press q in UI to stop host

# Non-blocking probes (CI seam — not the merge bar alone)
MCLOAD_STARTUP_PROBE=1 cargo run -- croft
MCLOAD_STARTUP_PROBE=1 cargo run -- loft --no-tray
```

PowerShell (Windows host) equivalents:

```powershell
cargo build
cargo run -- croft                 # press q to quit
cargo run -- loft                  # MUST show tray + BG; quiet unless --verbose; q stops host
cargo run -- loft --verbose
cargo run -- loft --no-tray        # foreground escape hatch
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- croft
$env:MCLOAD_STARTUP_PROBE = "1"; cargo run -- loft --no-tray
```

`--dry-run` remains available as a non-UI test seam:

```bash
cargo run -- croft --dry-run
cargo run -- loft --dry-run
```

### Optional Cargo features

| Feature      | Default | Notes                                                            |
| ------------ | ------- | ---------------------------------------------------------------- |
| `frankentui` | **on**  | Default feature; Croft/Loft start FrankenTUI without extra flags |

There is **no** top-level `tray` CLI mode or standalone `tray` Cargo feature. Tray support lives under Loft: default-on when the OS supports it; use `loft --no-tray` when it does not (or for WSL / headless CI). Do **not** require `loft --tray`.

### Line endings (LF)

The repo enforces LF via [`.gitattributes`](.gitattributes) (`* text=auto eol=lf`). For local clones, recommended (not committed):

```bash
git config --local core.autocrlf false
git config --local core.eol lf
```

<details>
<summary>Dev Container</summary>

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) (or compatible engine)
- [Dev Containers](https://containers.dev/) support in **VS Code / Cursor** (Dev Containers extension), **or** the [`devcontainer` CLI](https://github.com/devcontainers/cli)

The image is Rust-focused (`rust:bookworm` + `pkg-config` / build essentials) and matches `rust-toolchain.toml` (**stable**).

### Open / spin up (IDE)

1. Open the `mcload` folder in VS Code or Cursor.
2. Run **Dev Containers: Reopen in Container** (or **Open Folder in Container…**).
3. Wait for the image build and `postCreateCommand` (prints `rustc` / `cargo` versions).
4. Terminal inside the container is already at the workspace root — ready for `cargo test` / `cargo run`.

### Open / spin up (CLI)

From the repo root on the host:

```powershell
# PowerShell (Windows host)
devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . bash
```

```bash
# bash / macOS / Linux host
devcontainer up --workspace-folder .
devcontainer exec --workspace-folder . bash
```

Rebuild if the Dockerfile changes:

```powershell
devcontainer up --workspace-folder . --remove-existing-container
```

</details>

<details>
<summary>Running tests in container</summary>

After the container is up (IDE integrated terminal, or `devcontainer exec`):

```bash
# Full default-feature suite
cargo test

# Focused integration tests
cargo test --test cli_starts -- --nocapture
cargo test --test config_roundtrip -- --nocapture
cargo test --test metadata_concurrent -- --nocapture
cargo test --test ui_startup -- --nocapture

# Smoke the binary
cargo run -- --help
cargo run -- --version
MCLOAD_STARTUP_PROBE=1 cargo run -- croft
MCLOAD_STARTUP_PROBE=1 cargo run -- loft --no-tray
```

One-shot from the host without an interactive shell:

```powershell
devcontainer exec --workspace-folder . cargo test
```

**Note:** Interactive `cargo run -- croft` needs a real TTY. In headless `devcontainer exec` without a TTY, expect a clear `TtyUnavailable` error; use the probe env or an IDE terminal attached to a TTY. For Loft in containers / WSL, use `--no-tray` (foreground web server).

</details>

<details>
<summary>Multi-platform builds</summary>

Rebuild **all** documented release targets with the multi-build script (preferred before any manual test handoff):

```bash
# Linux / WSL / Dev Container
chmod +x scripts/build-release-multi.sh   # once
scripts/build-release-multi.sh
```

```powershell
# Windows (PowerShell)
pwsh -File scripts/build-release-multi.ps1
```

Artifacts land under `dist/release/<target>/mcload` (or `mcload.exe`). On Linux/WSL the script uses `x86_64-pc-windows-gnu` when MinGW is available instead of MSVC. macOS (`aarch64-apple-darwin`) is **skipped** unless you are on Apple Silicon (or set `USE_CROSS=1` / `-UseCross` with a working [cross](https://github.com/cross-rs/cross) toolchain). `STRICT=1` / `-Strict` treats skips as failure.

| Platform            | Rust target                | Typical notes                                                      |
| ------------------- | -------------------------- | ------------------------------------------------------------------ |
| Windows             | `x86_64-pc-windows-msvc`   | Native MSVC on Windows; GNU via MinGW when cross-building on Linux |
| Linux x86-64        | `x86_64-unknown-linux-gnu` | Native Linux, WSL, this Dev Container, or `cross`                  |
| macOS Apple Silicon | `aarch64-apple-darwin`     | Native on Apple Silicon; cross needs osxcross / special CI         |

### One-off per target (optional)

```powershell
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

From Windows without a Linux linker, prefer the Dev Container or `cross`:

```powershell
# after: cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target x86_64-unknown-linux-gnu
```

</details>
