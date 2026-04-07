use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetMode {
    Collapsed,
    Expanded,
    SearchMode,
    Minimized,
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationProgress {
    pub progress: f32,
    pub target: f32,
}

impl Default for AnimationProgress {
    fn default() -> Self {
        Self {
            progress: 0.0,
            target: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Trigger {
    Hover,
    Click,
    KeyPress,
    Escape,
    ClickOutside,
    SnapToCorner,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArtifactStatus {
    Idle,
    Starting,
    Running(String), // container id
    Error(String),
}

#[derive(Debug, Clone)]
pub struct State {
    pub mode: WidgetMode,
    pub animation_progress: AnimationProgress,
    pub input_text: String,
    pub is_hovered: bool,
    pub position: Position,
    pub corner_snap: Corner,
    pub is_animating: bool,
    pub show_create_menu: bool,
    pub show_recommendations: bool,
    pub recommendations_timer: f32,
    pub artifact_statuses: HashMap<String, ArtifactStatus>,
    pub setup_artifacts: Vec<String>,
    pub new_artifact_name: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: WidgetMode::Collapsed,
            animation_progress: AnimationProgress::default(),
            input_text: String::new(),
            is_hovered: false,
            position: Position::default(),
            corner_snap: Corner::None,
            is_animating: false,
            show_create_menu: false,
            show_recommendations: false,
            recommendations_timer: 0.0,
            artifact_statuses: {
                let mut m = HashMap::new();
                m.insert("File Manager".to_string(), ArtifactStatus::Starting);
                m
            },
            setup_artifacts: vec!["File Manager".to_string()], // Mock setup
            new_artifact_name: String::new(),
        }
    }
}

impl State {
    pub fn transition(&mut self, trigger: Trigger) {
        match (self.mode, trigger) {
            (WidgetMode::Collapsed, Trigger::Hover) => {
                self.mode = WidgetMode::Expanded;
                self.is_hovered = true;
            }
            (WidgetMode::Collapsed, Trigger::Click | Trigger::KeyPress) => {
                self.mode = WidgetMode::SearchMode;
            }
            (WidgetMode::Expanded, Trigger::Click | Trigger::KeyPress) => {
                self.mode = WidgetMode::SearchMode;
            }
            (WidgetMode::Expanded, Trigger::Escape | Trigger::Hover) => {
                self.mode = WidgetMode::Collapsed;
                self.is_hovered = false;
            }
            (WidgetMode::SearchMode, Trigger::Escape) => {
                self.input_text.clear();
                self.mode = WidgetMode::Collapsed;
            }
            (WidgetMode::SearchMode, Trigger::ClickOutside) => {
                self.mode = WidgetMode::Minimized;
            }
            (WidgetMode::Minimized, Trigger::Click) => {
                self.mode = WidgetMode::Collapsed;
            }
            (_, Trigger::SnapToCorner) => {
                self.corner_snap = Corner::TopRight;
            }
            _ => {}
        }
    }

    pub fn update_input(&mut self, c: char) {
        self.input_text.push(c);
        if self.mode == WidgetMode::Expanded {
            self.mode = WidgetMode::SearchMode;
        }
    }

    pub fn backspace(&mut self) {
        self.input_text.pop();
    }

    pub fn clear_input(&mut self) {
        self.input_text.clear();
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn center(
        screen_width: f32,
        screen_height: f32,
        widget_width: f32,
        widget_height: f32,
    ) -> Self {
        Self {
            x: (screen_width - widget_width) / 2.0,
            y: (screen_height - widget_height) / 2.0,
        }
    }

    pub fn top_right(
        screen_width: f32,
        _screen_height: f32,
        widget_width: f32,
        _widget_height: f32,
        margin: f32,
    ) -> Self {
        Self {
            x: screen_width - widget_width - margin,
            y: margin,
        }
    }

    pub fn bottom_center(
        screen_width: f32,
        screen_height: f32,
        widget_width: f32,
        widget_height: f32,
        margin: f32,
    ) -> Self {
        Self {
            x: (screen_width - widget_width) / 2.0,
            y: screen_height - widget_height - margin,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Corner {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
