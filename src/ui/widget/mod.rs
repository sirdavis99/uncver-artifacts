use iced::Subscription;
use std::sync::Arc;

pub const WINDOW_W: f32 = 480.0;
pub const WINDOW_H: f32 = 600.0;
use crate::artifacts::ArtifactManager;
use crate::ui::state::State;

pub mod update;
pub mod view;

pub struct SearchWidget {
    pub state: State,
    pub(crate) artifacts: Arc<ArtifactManager>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    ToggleSearch,
    Clear,
    ToggleCreateMenu,
    OpenCreateModal,
    CloseCreateModal,
    CreateTitleChanged(String),
    CreateDescriptionChanged(String),
    SelectCreateFolder,
    FolderSelected(Option<std::path::PathBuf>),
    SubmitCreateArtifact,
    OpenViewModal(String),
    OpenEditModal(String),
    SubmitEditArtifact,
    SubmitUpdateArtifact,
    ConfirmDeleteArtifact,
    CancelDeleteArtifact,
    SubmitDeleteArtifact,
    ArtifactCreated(String),
    ArtifactUpdated(std::path::PathBuf),
    OpenArtifact(String),
    ArtifactStarted(String, String),
    ArtifactError(String, String),
    ArtifactsLoaded(Vec<crate::artifacts::ArtifactConfig>),
    RefreshArtifacts,
    Tick(std::time::Instant),
    WindowEvent(iced::window::Id, iced::window::Event),
    None,
}

impl SearchWidget {
    pub fn new() -> Self {
        let artifacts = Arc::new(ArtifactManager::new().expect("Failed to init ArtifactManager"));
        Self {
            state: State::default(),
            artifacts,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![
            iced::time::every(std::time::Duration::from_millis(16)).map(Message::Tick),
            iced::window::events().map(|(id, event)| Message::WindowEvent(id, event)),
        ];

        // Add artifact watcher subscription
        subs.push(crate::artifacts::watcher::watch_artifacts().map(Message::ArtifactUpdated));

        Subscription::batch(subs)
    }
}
