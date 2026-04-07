# AGENTS.md — uncver-artifacts

> AI agent context file. Read this before making any changes to the project.

## Project Overview

**uncver-artifacts** is a macOS/cross-platform floating search widget with Podman integration, built in Rust using the Iced GUI framework.

- **Binary**: `uncver-artifacts` — a 400×48px frameless, transparent, bottom-centered HUD-style launcher
- **Purpose**: Minimal floating search UI that manages Podman containers via a system tray integration

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (Edition 2021) |
| GUI Framework | `iced` 0.14 (with `image`, `svg`, `tokio` features) |
| Async Runtime | `tokio` 1 (full features) |
| Window/Tray | `tao` 0.27, `winit` 0.29 |
| Error Handling | `anyhow` + `thiserror` |
| Logging | `tracing` + `tracing-subscriber` |
| Serialization | `serde` + `serde_json` |
| Container Engine | Podman (managed via CLI subprocess calls) |
| Platform-specific | `objc` (macOS), `winapi` (Windows) |

## Project Structure

```
src/
├── main.rs          # App entry point — window settings, iced::application wiring
├── lib.rs           # Crate root — re-exports SearchWidget
├── ui/
│   ├── mod.rs       # UI module exports
│   ├── state.rs     # SearchWidget state (search text, results, animation state)
│   └── widget.rs    # Main widget impl — update(), view(), messages
├── podman/
│   ├── mod.rs       # Podman facade + PodmanError enum
│   ├── install.rs   # PodmanInstaller — detects and installs Podman
│   ├── machine.rs   # PodmanMachine — manages podman machine lifecycle
│   └── runner.rs    # PodmanRunner — runs container images
└── tray/
    ├── mod.rs       # Tray module exports
    └── menu.rs      # System tray menu definition
```

## Key Architectural Decisions

1. **Fixed window size**: 400×48px, non-resizable, decorations off, transparent — intentional HUD design
2. **Tick-driven animation**: Subscription ticks at 16ms (~60fps) via `Message::Tick`
3. **Window events piped through messages**: `Message::WindowEvent(id, event)` for focus/blur handling
4. **macOS titlebar**: Hidden + fullsize content view for edge-to-edge rendering
5. **Podman abstraction**: `Podman` struct acts as a facade over install, machine, and runner submodules — all errors use `PodmanError` + `anyhow::Result`

## Key Patterns & Conventions

- All Rust modules use `pub mod` + re-exports in `mod.rs` (facade pattern)
- Error types defined with `thiserror::Error` derive
- Logging via `tracing::info!` / `tracing::debug!` — never use `println!` directly
- Async operations go through `tokio` — do not use blocking calls on the main thread
- Platform-specific code gated by `#[cfg(target_os = "macos")]` / `#[cfg(windows)]`

## Agent Rules

- Always run `cargo check` after any Rust edits to catch compile errors early
- Prefer `anyhow::Result` for fallible functions; use `thiserror` for library error types
- Keep `widget.rs` as the single source of truth for all UI state and messages
- Do not add new direct dependencies without checking `Cargo.toml` first

## Knowledge Links

| Topic | File |
|---|---|
| Podman integration patterns | `.agent/knowledge/podman.md` |
| UI widget & state patterns | `.agent/knowledge/ui.md` |
| Build & development setup | `.agent/knowledge/build.md` |
