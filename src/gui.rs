use crate::artifacts::ArtifactConfig;
use std::process::{Command, Stdio};

pub fn open_gui_window(artifact: &ArtifactConfig) {
    if let Some(gui) = &artifact.gui_window {
        if gui.enabled {
            // Traefik reverse proxy handles the routing via Host headers securely on a custom unprivileged port
            let domain = format!("{}.localhost", artifact.name.replace("uncver-", ""));
            let url = format!("http://{}:42080", domain);
            
            tracing::info!("Launching Native Webview to {}", url);
            
            let mut cmd = Command::new(std::env::current_exe().unwrap_or_else(|_| "uncver-artifacts".into()));
            cmd.arg("viewer").arg(&url);

            if let Some(w) = gui.width {
                cmd.arg("--width").arg(w.to_string());
            }
            if let Some(h) = gui.height {
                cmd.arg("--height").arg(h.to_string());
            }
            if let Some(x) = gui.x {
                cmd.arg("--x").arg(x.to_string());
            }
            if let Some(y) = gui.y {
                cmd.arg("--y").arg(y.to_string());
            }

            // Spawn the viewer as a detached child process process so the terminal can exit cleanly!
            let _ = cmd
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
    }
}

pub fn run_webview_viewer(url: &str, width: Option<u16>, height: Option<u16>, x: Option<i32>, y: Option<i32>) -> anyhow::Result<()> {
    use tao::{
        dpi::{PhysicalPosition, PhysicalSize},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use wry::WebViewBuilder;

    // Wait 2 seconds for the container service to boot before attaching the WebView!
    std::thread::sleep(std::time::Duration::from_secs(2));

    let event_loop = EventLoop::new();
    let mut builder = WindowBuilder::new()
        .with_title("uncver-artifacts")
        .with_decorations(false) // frameless native window!
        .with_transparent(true); // allow rounded corners via HTML CSS

    if let (Some(w), Some(h)) = (width, height) {
        builder = builder.with_inner_size(PhysicalSize::new(w as u32, h as u32));
    }
    if let (Some(px), Some(py)) = (x, y) {
        builder = builder.with_position(PhysicalPosition::new(px, py));
    }

    let window = builder.build(&event_loop)?;

    // Set Window Icon
    let icon_data = include_bytes!("../assets/icon.png");
    if let Ok(img) = image::load_from_memory(icon_data) {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        if let Ok(icon) = tao::window::Icon::from_rgba(rgba.into_raw(), width, height) {
            window.set_window_icon(Some(icon));
        }
    }

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let builder = WebViewBuilder::new().with_background_color((0, 0, 0, 0));
    
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        WebViewBuilder::new_gtk(window.gtk_window())
    };

    let _webview = builder
        .with_url(url)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });

    // unreachable!
}
