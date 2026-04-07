# UI Widget & State Patterns

> Knowledge module for `uncver-artifacts`. Covers the Iced-based UI architecture.

## Window Geometry

The app renders as a **400×48px** frameless, transparent, bottom-centered HUD:

```rust
// src/main.rs
size: Size::new(400.0, 48.0),
position: Position::SpecificWith(|_, monitor_size| {
    let x = (monitor_size.width - 400.0) / 2.0;
    let y = monitor_size.height - 48.0 - 50.0; // 50px from bottom
    iced::Point::new(x, y)
}),
decorations: false,
transparent: true,
```

macOS-specific: titlebar hidden + fullsize content view for edge-to-edge rendering.

## Architecture: `SearchWidget`

`SearchWidget` is the single app state. Source of truth lives in `src/ui/widget.rs`.

```
SearchWidget
├── .update(Message) -> Task<Message>
└── .view() -> Element<Message>
```

State fields live in `src/ui/state.rs` and are imported into `SearchWidget`.

## Message Enum

All app events flow through `Message` (defined in `widget.rs`):

| Variant | Purpose |
|---|---|
| `Tick` | 16ms animation tick (~60fps) |
| `WindowEvent(Id, Event)` | Raw window events (focus, blur, resize) |
| `SearchChanged(String)` | Text input updated |
| *(custom)* | Any new interaction must be added here |

**Rule**: `widget.rs` is the single source of truth. Do NOT define messages elsewhere.

## Subscription Pattern

```rust
// src/main.rs
fn subscription(_: &SearchWidget) -> iced::Subscription<Message> {
    let tick = iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick);
    let events = window::events().map(|(id, e)| Message::WindowEvent(id, e));
    iced::Subscription::batch([tick, events])
}
```

Add new subscriptions by extending this `batch`. Do not create separate subscription providers.

## Iced Version Notes

- **iced 0.14**: Uses `iced::application(new, update, view)` builder pattern (not `Sandbox` or `Application` traits)
- Features enabled: `image`, `svg`, `tokio`
- `Task<Message>` is the return type of `update` (replaces `Command` from older versions)

## Key Files

| File | Responsibility |
|---|---|
| `src/main.rs` | Window settings, subscription, app entry |
| `src/lib.rs` | Re-exports `SearchWidget` |
| `src/ui/state.rs` | State struct fields |
| `src/ui/widget.rs` | `SearchWidget` impl — update, view, messages |
| `src/ui/mod.rs` | Module re-exports |
