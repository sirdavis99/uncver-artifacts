use tracing::{error, info};

use uncver_artifacts::podman::Podman;

pub async fn upgrade(force: bool) -> anyhow::Result<()> {
    info!("Checking for updates...");
    match crate::upgrade::check_and_upgrade(force).await {
        Ok(msg) => println!("{}", msg),
        Err(e) => { error!("Upgrade failed: {}", e); std::process::exit(1); }
    }
    Ok(())
}

pub fn autostart(podman: &Podman, disable: bool) -> anyhow::Result<()> {
    if disable { podman.disable_autostart()?; println!("Podman auto-start disabled."); }
    else { podman.enable_autostart()?; println!("Podman auto-start enabled."); }
    Ok(())
}

#[cfg(feature = "gui")]
pub fn viewer(url: &str, width: Option<u16>, height: Option<u16>, x: Option<i32>, y: Option<i32>) -> anyhow::Result<()> {
    uncver_artifacts::gui::run_webview_viewer(url, width, height, x, y)
}

#[cfg(feature = "gui")]
pub fn tray() -> anyhow::Result<()> {
    uncver_artifacts::tray::run_tray()
}
