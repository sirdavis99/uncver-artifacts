use std::collections::HashMap;
use std::time::Instant;
use crate::artifacts::ArtifactConfig;

pub mod mode;
pub mod animation;
pub mod trigger;
pub mod status;
pub mod persistent;

pub use mode::WidgetMode;
pub use animation::AnimationProgress;
pub use trigger::Trigger;
pub use status::ArtifactStatus;
pub use persistent::PersistentState;

pub struct State {
    pub mode: WidgetMode,
    pub input_text: String,
    pub animation_progress: AnimationProgress,
    pub trigger: Trigger,
    pub last_tick: Instant,
    pub show_recommendations: bool,
    pub show_create_menu: bool,
    pub show_create_modal: bool,
    pub create_form_title: String,
    pub create_form_description: String,
    pub create_form_folder: Option<std::path::PathBuf>,
    pub recommendations_timer: f32,
    pub artifacts: Vec<ArtifactConfig>,
    pub artifact_statuses: HashMap<String, ArtifactStatus>,
    pub is_loading: bool,
    pub is_hovered: bool,
    pub selected_artifact: Option<String>,
    pub is_viewing: bool,
}

impl Default for State {
    fn default() -> Self {
        let persistent = PersistentState::load();
        Self {
            mode: WidgetMode::Collapsed,
            input_text: persistent.input_text,
            animation_progress: AnimationProgress::default(),
            trigger: Trigger::Manual,
            last_tick: Instant::now(),
            show_recommendations: false,
            show_create_menu: false,
            show_create_modal: false,
            create_form_title: String::new(),
            create_form_description: String::new(),
            create_form_folder: None,
            recommendations_timer: 0.0,
            artifacts: Vec::new(),
            artifact_statuses: HashMap::new(),
            is_loading: true, // Trigger initial load on startup
            is_hovered: false,
            selected_artifact: None,
            is_viewing: false,
        }
    }
}

impl State {
    pub fn save_persistent(&self) {
        let ps = PersistentState {
            input_text: self.input_text.clone(),
        };
        ps.save();
    }

    pub fn clear_input(&mut self) {
        self.input_text.clear();
        self.save_persistent();
    }

    pub fn reset_create_form(&mut self) {
        self.create_form_title.clear();
        self.create_form_description.clear();
        self.create_form_folder = None;
        self.selected_artifact = None;
        self.is_viewing = false;
    }
}
