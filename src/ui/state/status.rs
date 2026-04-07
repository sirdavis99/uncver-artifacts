#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactStatus {
    Idle,
    Starting,
    Running(String),
    Stopping,
    Error(String),
}
