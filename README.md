# McLoad

> There can be only one!

**McLoad** is a single-binary Rust tool for filesystem load / metadata workflows with one Source of Truth in mind. It exposes three launch modes — **Croft** (TTY UI), **Loft** (Web UI), and **tray/background** — plus YAML config, a concurrent metadata store API, and queue stubs (Activity / Gathering / Reckoning). Scaffolding ships UI and tray as stubs; real FrankenTUI and tray backends are optional features.

Licensed under the [MIT License](LICENSE).

## Usage

Build and run from the repo root (Rust stable; see `rust-toolchain.toml`):

```bash
cargo build
cargo run -- --help
cargo run -- --version
```

| Mode / flag           | Purpose                                              |
|-----------------------|------------------------------------------------------|
| `mcload --help`       | Print CLI help and exit                              |
| `mcload --version`    | Print version (`CARGO_PKG_VERSION`) and exit         |
| `mcload croft`        | Croft — FrankenTUI TTY UI (stub by default)          |
| `mcload loft`         | Loft — FrankenTUI Web/WASM UI (stub by default)      |
| `mcload tray`         | Tray / background mode (stub; optional `tray` feat.) |
| `… --dry-run`         | Mode seam: exit 0 without opening UI/tray            |
| `--config <path>`     | Override config YAML path (global)                   |
| `--project <dir>`     | Per-project config directory (global)                |
| `--log-level <level>` | Override log level from CLI (global)                 |

Examples:

```bash
cargo run -- croft --dry-run
cargo run -- loft --dry-run
cargo run -- tray --dry-run
```

Optional Cargo features:

| Feature      | Default | Notes                                           |
|--------------|---------|-------------------------------------------------|
| `tray`       | off     | Native tray deps when implementing the stub     |
| `frankentui` | off     | Real FrankenTUI imports when deps are available |

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

# Smoke the binary
cargo run -- --help
cargo run -- --version
cargo run -- croft --dry-run
cargo run -- loft --dry-run
cargo run -- tray --dry-run
```

One-shot from the host without an interactive shell:

```powershell
devcontainer exec --workspace-folder . cargo test
```

</details>

<details>
<summary>Multi-platform builds</summary>

Scaffolding documents the target matrix; producing every artifact usually needs a native host or [cross](https://github.com/cross-rs/cross). Host smoke: `cargo check` / `cargo build --release` on the machine you have.

| Platform            | Rust target                | Typical notes                                                                 |
|---------------------|----------------------------|-------------------------------------------------------------------------------|
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
