# AGENTS.md — uncver-artifacts (CLI Version)

> AI agent context file. Read this before making any changes to the project.

## Project Overview

**uncver-artifacts** is a CLI tool for managing artifacts with Podman integration, built in Rust.

- **Binary**: `uncver-artifacts` — a command-line interface for artifact management
- **Purpose**: Manage Podman containers and artifacts via CLI commands

## Tech Stack

| Layer              | Technology                                        |
|--------------------|---------------------------------------------------|
| Language           | Rust (Edition 2021)                               |
| Async Runtime      | `tokio` 1 (full features)                         |
| CLI Framework      | `clap` 4.5 (derive features)                      |
| Error Handling     | `anyhow` + `thiserror`                            |
| Logging            | `tracing` + `tracing-subscriber`                  |
| Serialization      | `serde` + `serde_json`                            |
| Container Engine   | Podman (managed via CLI subprocess calls)         |
| File Watching      | `notify` 8.2.0                                    |

## Project Structure

```bash
src/
├── main.rs          # CLI entry point — argument parsing, command dispatch
├── lib.rs           # Crate root — re-exports modules
├── artifacts/       # Artifact management module
│   ├── mod.rs       # ArtifactConfig, ArtifactManager
│   ├── builder.rs   # Build artifacts from config
│   └── watcher.rs   # File system watcher for artifacts
└── podman/          # Podman integration module
    ├── mod.rs       # Podman facade + PodmanError enum
    ├── install.rs   # PodmanInstaller — detects and installs Podman
    ├── machine.rs   # PodmanMachine — manages podman machine lifecycle
    └── runner.rs    # PodmanRunner — runs container images
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `install` | Install and setup Podman dependencies |
| `list` | List all artifacts |
| `start <name>` | Start an artifact by name |
| `create` | Create a new artifact with options |
| `delete <name>` | Delete an artifact |
| `watch` | Watch artifacts directory for changes |
| `run` | Run all default artifacts |

## Key Architectural Decisions

1. **CLI-only**: No GUI - all interaction through command line
2. **Podman abstraction**: `Podman` struct acts as a facade over install, machine, and runner submodules
3. **Artifact storage**: Artifacts stored in `~/.local/share/uncver-artifacts/artifacts/` (platform-specific data dir)
4. **Artifact format**: Each artifact is a folder with `artifact.json` containing metadata

## Artifact JSON Format

```json
{
  "name": "my-artifact",
  "description": "Optional description",
  "url": "https://github.com/user/repo",
  "local_path": "/path/to/code",
  "container_image": "docker.io/myimage:latest"
}
```

## Key Patterns & Conventions

- All Rust modules use `pub mod` + re-exports in `mod.rs` (facade pattern)
- Error types defined with `thiserror::Error` derive
- Logging via `tracing::info!` / `tracing::debug!` — never use `println!` directly
- Async operations go through `tokio` — do not use blocking calls on the main thread
- CLI output uses `println!` for user-facing messages, `tracing` for diagnostics

## Agent Rules

- Always run `cargo check` after any Rust edits to catch compile errors early
- Prefer `anyhow::Result` for fallible functions; use `thiserror` for library error types
- Keep commands modular - each subcommand should be self-contained
- Do not add new direct dependencies without checking `Cargo.toml` first

## Related Projects

| Project | Purpose |
|---------|---------|
| `uncver-create-artifact` | C++ tool for creating artifact.json files |
| `uncver-redis-stream-artifact` | Redis stream artifact implementation |
