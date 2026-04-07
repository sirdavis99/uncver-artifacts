pub mod menu;

pub use menu::TrayMenu;

pub struct Tray;

impl Tray {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Tray {
    fn default() -> Self {
        Self::new()
    }
}

pub enum TrayEvent {
    Show,
    Hide,
    Quit,
}
