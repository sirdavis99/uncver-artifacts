# AGENTS.md — uncver-artifacts

> AI agent context file. Read this before making any changes to the project.

## Project Overview

**uncver-artifacts** is a macOS/cross-platform floating search widget with Podman integration, built in Rust using the Iced GUI framework.

- **Binary**: `uncver-artifacts` — a 400×48px frameless, transparent, bottom-centered HUD-style launcher
- **Purpose**: Minimal floating search UI that manages Podman containers via a system tray integration

## Tech Stack

| Layer              | Technology                                        |
|--------------------|---------------------------------------------------|
| Language           | Rust (Edition 2021)                               |
| GUI Framework      | `iced` 0.14 (with `image`, `svg`, `tokio` features) |
| Async Runtime      | `tokio` 1 (full features)                         |
| Window/Tray        | `tao` 0.27, `winit` 0.29                          |
| Error Handling     | `anyhow` + `thiserror`                            |
| Logging            | `tracing` + `tracing-subscriber`                  |
| Serialization      | `serde` + `serde_json`                            |
| Container Engine   | Podman (managed via CLI subprocess calls)         |
| Platform-specific  | `objc` (macOS), `winapi` (Windows)                |

## Project Structure

```bash
src/
├── main.rs          # App entry point — window settings, iced::application wiring
├── lib.rs           # Crate root — re-exports SearchWidget
├── ui/
│   ├── mod.rs       # UI module exports
│   ├── state/
│   │   └── mod.rs   # SearchWidget state (input, artifacts, animation, modal flags)
│   ├── widget/
│   │   ├── mod.rs   # SearchWidget impl + Message enum
│   │   ├── update.rs # Message handler — all state transitions
│   │   └── view.rs  # View fn — dispatches Create/View/Edit modals via state flags
│   └── components/
│       ├── mod.rs          # Re-exports all components (facade)
│       ├── modal.rs        # modal_frame — shared styled container shell
│       ├── modal_button.rs # Reusable buttons: primary, secondary, blue, danger, ghost, disabled
│       ├── view_modal.rs   # Artifact view (read-only) modal
│       ├── edit_modal.rs   # Artifact edit modal (fields + save/remove)
│       ├── create_modal.rs # New artifact creation modal
│       ├── artifact.rs     # artifact_item (card row), artifact_card, plus_icon_button
│       ├── search.rs       # search_bar, search_icon_button, clear_button
│       ├── menu.rs         # create_artifact_menu dropdown
│       ├── menu_item.rs    # menu_button reusable item
│       └── icons.rs        # Icon helpers
├── podman/
│   ├── mod.rs       # Podman facade + PodmanError enum
│   ├── install.rs   # PodmanInstaller — detects and installs Podman
│   ├── machine.rs   # PodmanMachine — manages podman machine lifecycle
│   └── runner.rs    # PodmanRunner — runs container images
└── tray/
    ├── mod.rs       # Tray module exports
    └── menu.rs      # System tray menu definition
```

## Artifact UI — Modal System

### Modal Dispatch (view.rs)
The `show_create_modal` flag gates all modals. Which modal renders depends on two state flags:

| `is_viewing` | `selected_artifact` | Modal rendered      |
|:---:|:---:|:---|
| `true`  | any   | `view_artifact_modal`   — read-only details |
| `false` | `Some` | `edit_artifact_modal`  — editable fields    |
| `false` | `None` | `create_artifact_modal_view` — new artifact |

### Interaction Flow
```
Click artifact card  →  OpenViewModal(name)
  └─ view modal:  [✕ Close Icon in Header]
       └─ [DESCRIPTION] labels above content
       └─ [FOLDER] labels above content
       └─ [ ▶ Start Artifact ] (pill, centered primary action)
       └─ [Edit Artifact] (shrink width, aligned RIGHT in footer)
  
Click Edit/Settings  →  SubmitEditArtifact  →  edit modal
  └─ [Cancel] (left) ──── [Save Changes] (right)
  └─ [Remove Artifact] (full-width, below)

Click "+" button  →  ShowCreateModal
  └─ [Cancel] (left) ──── [Create Artifact] (right)
```

### Button Layout Convention
- **Cancel / Close always on the LEFT** (except for X close icon in header)
- **Primary action (Create / Save) always on the RIGHT**
- **Secondary actions (Edit)** can be right-aligned in the view footer
- Buttons use a `[ghost LEFT] spacer [action RIGHT]` row pattern in footers
- Remove (destructive) is a full-width standalone row — never paired with Cancel

### Shared Primitives (modal_button.rs)
| Function | Width | Purpose |
|---|---|---|
| `primary_btn` | Fill | Green — Create / Start |
| `blue_btn`    | Fill | Blue — Save Changes |
| `secondary_btn`| Shrink| Grey bordered — Edit / secondary |
| `danger_btn` | Fill | Red — Remove |
| `ghost_btn` | Shrink| No bg — Cancel / Close |
| `disabled_btn`| Fill | Grey container — loading state |
| `close_icon_btn`| - | Circular X for headers |

## Key Architectural Decisions

1. **Fixed window size**: 400×48px, non-resizable, decorations off, transparent — intentional HUD design
2. **Tick-driven animation**: Subscription ticks at 16ms (~60fps) via `Message::Tick`
3. **Window events piped through messages**: `Message::WindowEvent(id, event)` for focus/blur handling
4. **macOS titlebar**: Hidden + fullsize content view for edge-to-edge rendering
5. **Podman abstraction**: `Podman` struct acts as a facade over install, machine, and runner submodules — all errors use `PodmanError` + `anyhow::Result`
6. **Modal scrim**: A semi-transparent dark layer (`Color::from_rgba(0,0,0, 0.18*alpha)`) is stacked under all modals for depth — use `stack![main_view, scrim, overlay]`

## Key Patterns & Conventions

- All Rust modules use `pub mod` + re-exports in `mod.rs` (facade pattern)
- Error types defined with `thiserror::Error` derive
- Logging via `tracing::info!` / `tracing::debug!` — never use `println!` directly
- Async operations go through `tokio` — do not use blocking calls on the main thread
- Platform-specific code gated by `#[cfg(target_os = "macos")]` / `#[cfg(windows)]`
- **Font weight**: Use `Weight::Semibold` (not `SemiBold`) — iced 0.14 casing
- **Space::new()** takes no arguments in iced 0.14

## Agent Rules

- Always run `cargo check` after any Rust edits to catch compile errors early
- Prefer `anyhow::Result` for fallible functions; use `thiserror` for library error types
- Keep `widget.rs` as the single source of truth for all UI state and messages
- **Modular UI Components**: Keep `src/ui/components/` split into small, logical files. Max ~150 lines per file.
- Do not add new direct dependencies without checking `Cargo.toml` first
- **Modal buttons**: Always use primitives from `modal_button.rs` — do not inline button styles ad-hoc

## Knowledge Links

| Topic                        | File                          |
|------------------------------|-------------------------------|
| Podman integration patterns  | `.agent/knowledge/podman.md`  |
| UI widget & state patterns   | `.agent/knowledge/ui.md`      |
| Build & development setup    | `.agent/knowledge/build.md`   |
