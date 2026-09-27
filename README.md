# McLoad

> There can be only one!

**McLoad** is a single-binary Rust tool for filesystem load / metadata workflows with one Source of Truth in mind. It exposes two launch modes — **Croft** (FrankenTUI TTY UI) and **Loft** (FrankenTUI Web UI) — plus YAML config, a concurrent metadata store API, and queue stubs (Activity / Gathering / Reckoning). Tray (when the OS supports it) is an optional capability **under Loft**, not a separate CLI mode.

Licensed under the [MIT License](LICENSE).

## Usage

Build and run from the repo root (Rust stable; see `rust-toolchain.toml`):

```bash
cargo build
cargo run -- --help
cargo run -- --version
```

| Mode / flag           | Purpose                                                               |
| --------------------- | --------------------------------------------------------------------- |
| `mcload --help`       | Print CLI help and exit                                               |
| `mcload --version`    | Print version (`CARGO_PKG_VERSION`) and exit                          |
| `mcload croft`        | Croft — FrankenTUI TTY UI (needs a real TTY; press `q` to quit)       |
| `mcload loft`         | Loft — FrankenTUI Web UI (prints a local `http://127.0.0.1:PORT` URL) |
| `… --dry-run`         | Test seam only: exit 0 without opening UI (not merge / UI acceptance) |
| `--config <path>`     | Override config YAML path (global)                                    |
| `--project <dir>`     | Per-project config directory (global)                                 |
| `--log-level <level>` | Override log level from CLI (global)                                  |

**Croft** requires an interactive TTY. Without one it exits with a clear `TtyUnavailable` error (re-run in a real terminal, or use `--dry-run` / `MCLOAD_STARTUP_PROBE=1`). **Loft** binds a local HTTP listener and prints the URL to stderr; open it in a browser and press Ctrl+C to stop.

### Paste-ready startup (Croft + Loft)

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

`--dry-run` remains available as a non-UI test seam:

```bash
cargo run -- croft --dry-run
cargo run -- loft --dry-run
```

### Optional Cargo features

| Feature      | Default | Notes                                                            |
| ------------ | ------- | ---------------------------------------------------------------- |
| `frankentui` | **on**  | Default feature; Croft/Loft start FrankenTUI without extra flags |

There is **no** top-level `tray` CLI mode or standalone `tray` Cargo feature. Tray support (if added later) lives under Loft when the OS allows it.

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
MCLOAD_STARTUP_PROBE=1 cargo run -- loft
```

One-shot from the host without an interactive shell:

```powershell
devcontainer exec --workspace-folder . cargo test
```

**Note:** Interactive `cargo run -- croft` needs a real TTY. In headless `devcontainer exec` without a TTY, expect a clear `TtyUnavailable` error; use the probe env or an IDE terminal attached to a TTY.

</details>

<details>
<summary>Multi-platform builds</summary>

Scaffolding documents the target matrix; producing every artifact usually needs a native host or [cross](https://github.com/cross-rs/cross). Host smoke: `cargo check` / `cargo build --release` on the machine you have.

| Platform            | Rust target                | Typical notes                                                                 |
| ------------------- | -------------------------- | ----------------------------------------------------------------------------- |
| Windows             | `x86_64-pc-windows-msvc`   | Native MSVC toolchain on Windows                                              |
| Linux x86-64        | `x86_64-unknown-linux-gnu` | Native Linux, WSL, this Dev Container, or `cross` from another OS             |
| macOS Apple Silicon | `aarch64-apple-darwin`     | Native on Apple Silicon; cross from Linux/Windows needs osxcross / special CI |

### Windows (MSVC)

```powershell
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

### Linux x86-64

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

From Windows without a Linux linker, prefer the Dev Container or `cross`:

```powershell
# after: cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target x86_64-unknown-linux-gnu
```

### macOS Apple Silicon

```bash
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

Cross-compiling *to* macOS from Linux/Windows is non-trivial (Apple SDK / linker). Prefer a macOS host or documented CI runners for release artifacts.

</details>
