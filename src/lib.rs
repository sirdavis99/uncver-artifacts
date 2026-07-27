pub mod artifacts;
pub mod output;
#[cfg(feature = "gui")]
pub mod gui;
pub mod paths;
pub mod podman;
#[cfg(feature = "gui")]
pub mod tray;
pub mod upgrade;

pub use artifacts::{ArtifactConfig, ArtifactManager};
pub use podman::Podman;
#[cfg(feature = "gui")]
pub use gui::open_gui_window;
pub use upgrade::UpgradeManager;

#[cfg(not(feature = "gui"))]
pub fn open_gui_window(_artifact: &ArtifactConfig) {
    // GUI not enabled — no-op
}
