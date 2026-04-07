#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    Manual,
    SystemTray,
    Shortcut,
}
