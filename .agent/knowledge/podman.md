# Podman Integration Patterns

> Knowledge module for `uncver-artifacts`. Covers how Podman is detected, installed, and operated.

## Architecture

The Podman layer is a **facade pattern**:

```
Podman (facade, src/podman/mod.rs)
├── PodmanInstaller (src/podman/install.rs)  — detect + install
├── PodmanMachine   (src/podman/machine.rs)  — VM lifecycle
└── PodmanRunner    (src/podman/runner.rs)   — container execution
```

The outer `Podman` struct owns all three and exposes high-level methods:
- `ensure_installed()` — checks presence, auto-installs if missing
- `ensure_machine_running()` — checks VM state, starts if stopped
- `run(image)` — delegates to `PodmanRunner`

## Error Handling

All errors flow through `PodmanError` (defined in `mod.rs`):

```rust
pub enum PodmanError {
    NotInstalled,
    MachineError(String),
    RunError(String),
    InstallError(String),
}
```

- Use `anyhow::Result<T>` for return types in all public methods
- Wrap `PodmanError` variants via `.map_err(|e| anyhow::anyhow!(e))`

## CLI Subprocess Pattern

All Podman operations use `std::process::Command` (synchronous). The pattern:

```rust
let output = Command::new("podman")
    .args(&["machine", "start"])
    .output()?;

if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(PodmanError::MachineError(stderr.to_string()).into());
}
```

**Important**: These are blocking calls. If called from an async context, use `tokio::task::spawn_blocking`.

## Installation Detection

`PodmanInstaller::is_installed()` checks for the `podman` binary in `PATH` via:

```rust
Command::new("podman").arg("--version").output()
```

On macOS, Podman can be installed via Homebrew (`brew install podman`) or the official `.pkg` installer.

## Key Files

| File | Responsibility |
|---|---|
| `src/podman/mod.rs` | Facade struct + `PodmanError` enum |
| `src/podman/install.rs` | Binary detection + installation automation |
| `src/podman/machine.rs` | `podman machine start/stop/status` |
| `src/podman/runner.rs` | `podman run <image>` invocation |
