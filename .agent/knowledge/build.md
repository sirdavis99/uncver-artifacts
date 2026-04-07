# Build & Development Setup

> Knowledge module for `uncver-artifacts`. Covers how to build, run, and develop locally.

## Quick Start

```bash
# Check for compile errors (run after every edit)
cargo check

# Build debug binary
cargo build

# Run the app (debug mode)
cargo run

# Build release binary
cargo build --release
```

The release binary is at: `target/release/uncver-artifacts`

## Hot Reload

A hot-reload script is provided at `hot-reload.sh`:

```bash
./hot-reload.sh
```

This script watches source files and re-runs `cargo run` on changes. Requires `cargo-watch` to be installed:

```bash
cargo install cargo-watch
```

## Key Dependencies (Cargo.toml)

| Crate | Version | Notes |
|---|---|---|
| `iced` | 0.14 | GUI framework — features: `image`, `svg`, `tokio` |
| `tao` | 0.27 | Window/event loop |
| `winit` | 0.29 | Window management |
| `tokio` | 1 | Async runtime (full features) |
| `anyhow` | 1.0 | Error propagation |
| `thiserror` | 1.0 | Error type derivation |
| `tracing` | 0.1 | Structured logging |
| `serde` + `serde_json` | 1.0 | Serialization |
| `dirs` | 5.0 | XDG/platform directories |
| `reqwest` | 0.12 | HTTP (blocking feature) |

Platform-specific:
- macOS: `objc` 0.2
- Windows: `winapi` 0.3

## Adding Dependencies

1. Check `Cargo.toml` first — do not duplicate existing crates
2. Prefer pinning minor version: `crate = "X.Y"` not `"*"`
3. Use `cargo check` after adding to validate resolution
4. If platform-specific: gate under `[target.'cfg(target_os = "...")'.dependencies]`

## Logging

The app uses `tracing` with env-filter:

```rust
tracing_subscriber::fmt()
    .with_env_filter("uncver_artifacts=debug,info")
    .init();
```

To increase verbosity at runtime:
```bash
RUST_LOG=debug cargo run
```

**Never use `println!`** — always use `tracing::info!`, `tracing::debug!`, `tracing::warn!`, `tracing::error!`.

## Build Script

`build.rs` is present but minimal. Check it before adding platform-specific compile-time configuration.

## CI / Quality Gates

- `cargo check` — fast syntax + type check (required after every edit)
- `cargo clippy` — linting (run before PRs)
- `cargo test` — unit tests
- `cargo build --release` — final validation
