# AGENTS.md — uncver-artifacts

> AI agent context file. Read this before making any changes to the project.

## Project Overview

**uncver-artifacts** is a CLI tool for managing Podman containers from artifact definitions pulled dynamically from `uncoverthefuture-org/uncver-artifact-lib`.

- **Binary**: `uncver-artifacts` — CLI for artifact/container lifecycle
- **Purpose**: On `install`, clones the artifact-lib repo, reads `required_artifacts` with `auto_start: true`, builds/starts them all on `uncver-network`

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
| GUI (optional)     | `tao` + `wry` (native webview), `tray-icon`       |
| HTTP Client        | `reqwest` (self-upgrade checks)                   |

## Project Structure

```bash
src/
├── main.rs              # CLI dispatch (~87 lines) — clap parse + handler calls
├── lib.rs               # Crate root — re-exports modules
├── handlers/            # Subcommand handler functions (split by domain)
│   ├── mod.rs           # Module declarations only
│   ├── install.rs       # install, refresh
│   ├── container.rs     # start, run, ps, logs, reset
│   ├── artifact.rs      # list, create, delete, watch, load
│   └── system.rs        # upgrade, autostart, tray, viewer
├── install.rs           # artifact-lib sync, ensure_required_artifacts, binary registration
├── paths.rs             # Centralized path helpers for data dirs
├── gui.rs               # Native webview via tao + wry (gui feature)
├── tray.rs              # System tray (gui feature)
├── artifacts/           # Artifact management module
│   ├── mod.rs           # ArtifactConfig, ArtifactManager
│   ├── builder.rs       # Build artifacts from config
│   └── watcher.rs       # File system watcher for artifacts
├── podman/              # Podman integration module
│   ├── mod.rs           # Podman facade + PodmanError enum
│   ├── install.rs       # PodmanInstaller — detects and installs Podman
│   ├── machine.rs       # PodmanMachine — manages podman machine lifecycle
│   └── runner.rs        # PodmanRunner — runs container images
├── traefik/             # Traefik reverse-proxy orchestrator
└── upgrade/             # Self-upgrade module (GitHub release checks)
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `install` | Install Podman, clone artifact-lib, build/start required artifacts |
| `refresh` | Pull latest artifact-lib, rebuild/restart required artifacts |
| `list` | List all registered artifacts |
| `start <name>` | Start an artifact by name |
| `create` | Create a new artifact with options |
| `delete <name>` | Delete an artifact |
| `watch` | Watch artifacts directory for changes |
| `run` | Run all artifacts |
| `load <path>` | Load and start an artifact from a local directory with artifact.json |
| `upgrade` | Self-upgrade to the latest release |
| `reset` | Remove all uncver containers, re-create Traefik + Redis |
| `autostart` | Enable/disable Podman machine auto-start on boot |
| `ps` | List all podman containers |
| `logs <name>` | View logs for a specific container |
| `tray` | Start system tray (gui feature) |
| `viewer <url>` | Launch a native webview window (gui feature, internal) |

## Key Architectural Decisions

1. **No host processes** — Redis, AI router, and all artifacts run as Podman containers on `uncver-network`
2. **Dynamic artifact discovery** — `install`/`refresh` clone/pull `uncver-artifact-lib`, read root `artifact.json`, process only `required_artifacts` with `auto_start: true`
3. **Handlers module pattern** — Each command group has its own file under `handlers/`, main.rs is a thin dispatch layer (~87 lines)
4. **GUI via separate binary call** — `open_gui_window()` spawns `uncver-artifacts viewer <url>` as a detached child process; no blocking the terminal
5. **Podman abstraction** — `Podman` struct acts as a facade over install, machine, and runner submodules
6. **Artifact storage** — Artifacts stored in platform-specific data dirs (`dirs` crate)
7. **Build-time GUI deps** — `pkg-config`, `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libxdo-dev` auto-installed by `install` command

## Artifact JSON Format (artifact-lib)

```json
{
  "name": "uncver-redis-stream",
  "container_image": "docker.io/library/redis:7-alpine",
  "image_source": "registry",
  "ports": ["6379:6379"],
  "environment": {},
  "volumes": ["vol-name:/container/path"],
  "network": "uncver-network",
  "build": {
    "steps": ["podman build -t my-image ."]
  }
}
```

### Image resolution

`ensure_required_artifacts` tries to pull `container_image` first; if pull fails and the artifact directory has a Dockerfile, it falls back to building locally. If `container_image` is empty, it builds from the Dockerfile. No explicit field needed — the system handles it automatically.

## Key Patterns & Conventions

- All Rust modules use `pub mod` + re-exports in `mod.rs` (facade pattern)
- Handler files in `handlers/` expose standalone `pub fn` — no trait impls on `Commands`
- Error types defined with `thiserror::Error` derive
- Logging via `tracing::info!` / `tracing::debug!` — never use `println!` directly
- CLI output uses `println!` for user-facing messages, `tracing` for diagnostics
- `anyhow::Result` for fallible functions; `thiserror` for library error types
- Keep commands modular — each subcommand should be self-contained
- Do not add new direct dependencies without checking `Cargo.toml` first

## Agent Rules

- Always run `cargo check` after any Rust edits to catch compile errors early
- After `cargo check`, also run `cargo check --no-default-features` to verify no-GUI build
- The project has `default = ["gui"]` in Cargo.toml — GUI feature includes `tray-icon`, `tao`, `wry`, `image`

## Related Repositories

| Project | Purpose |
|---------|---------|
| `uncoverthefuture-org/uncver-artifact-lib` | Central artifact registry — root `artifact.json` declares `required_artifacts` |
| `sirdavis99/ollama-qwen3-0.6b-ai` | AI router TypeScript app (embedded Ollama), referenced by artifact-lib |
