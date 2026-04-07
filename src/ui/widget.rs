use iced::widget::{button, column, container, row, text, Row, Space};
use iced::{window, Alignment, Color, Element, Length, Padding, Task};
use tracing::info;
use rfd::FileDialog;
use std::path::PathBuf;

use crate::ui::state::{State, ArtifactStatus};
use crate::ui::components;
use crate::artifacts::{ArtifactConfig, ArtifactManager};

pub const WINDOW_W: f32 = 460.0;
pub const WINDOW_H: f32 = 600.0;

pub struct SearchWidget {
    pub state: State,
    pub artifacts: ArtifactManager,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    ToggleSearch,
    Clear,
    ToggleCreateMenu,
    CreateArtifact,
    ArtifactCreated(String),
    ArtifactUpdated(PathBuf),
    OpenArtifact(String),
    ArtifactStarted(String, String),
    ArtifactError(String, String),
    Tick(std::time::Instant),
    WindowEvent(window::Id, window::Event),
    None,
}

impl SearchWidget {
    pub fn new() -> (Self, Task<Message>) {
        let artifacts = ArtifactManager::new().expect("Failed to initialize ArtifactManager");
        (
            Self {
                state: State::default(),
                artifacts,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(val) => {
                self.state.input_text = val;
                Task::none()
            }
            Message::ToggleSearch => {
                if self.state.mode == crate::ui::state::WidgetMode::Collapsed {
                    self.state.mode = crate::ui::state::WidgetMode::SearchMode;
                } else {
                    self.state.mode = crate::ui::state::WidgetMode::Collapsed;
                    self.state.input_text.clear();
                }
                Task::none()
            }
            Message::Clear => {
                self.state.clear_input();
                Task::none()
            }
            Message::ToggleCreateMenu => {
                self.state.show_create_menu = !self.state.show_create_menu;
                Task::none()
            }
            Message::CreateArtifact => {
                self.state.show_create_menu = false;
                let manager = self.artifacts.clone();
                Task::perform(
                    async move {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            let name = format!("Artifact-{}", path.file_name().unwrap_or_default().to_string_lossy());
                            let config = ArtifactConfig {
                                name: name.clone(),
                                description: None,
                                url: None,
                                local_path: Some(path.to_string_lossy().to_string()),
                                container_image: Some("ghcr.io/podman/hello:latest".to_string()),
                            };
                            if let Ok(_) = manager.create_artifact(&config) {
                                return Some(name);
                            }
                        }
                        None
                    },
                    |res| {
                        if let Some(name) = res {
                            Message::ArtifactCreated(name)
                        } else {
                            Message::None
                        }
                    }
                )
            }
            Message::ArtifactCreated(_name) => {
                info!("Artifact created!");
                Task::none()
            }
            Message::ArtifactUpdated(_path) => {
                info!("Artifact file updated!");
                Task::none()
            }
            Message::OpenArtifact(name) => {
                let status = self.state.artifact_statuses.get(&name).cloned().unwrap_or(ArtifactStatus::Idle);
                
                match status {
                    ArtifactStatus::Idle => {
                        self.state.artifact_statuses.insert(name.clone(), ArtifactStatus::Starting);
                        let name_clone = name.clone();
                        let manager = self.artifacts.clone();
                        
                        Task::perform(
                            async move {
                                let artifacts = manager.list_artifacts().await.unwrap_or_default();
                                if let Some(artifact) = artifacts.into_iter().find(|a| a.name == name_clone) {
                                    let runner = crate::podman::runner::PodmanRunner::new();
                                    let image = artifact.container_image.unwrap_or_else(|| "ghcr.io/podman/hello:latest".to_string());
                                    
                                    let res = tokio::task::spawn_blocking(move || {
                                        runner.run(&image)
                                    }).await;

                                    match res {
                                        Ok(Ok(id)) => Ok(id),
                                        Ok(Err(e)) => Err(e.to_string()),
                                        Err(e) => Err(e.to_string()),
                                    }
                                } else {
                                    Err("Artifact not found".to_string())
                                }
                            },
                            move |res| match res {
                                Ok(id) => Message::ArtifactStarted(name.clone(), id),
                                Err(e) => Message::ArtifactError(name.clone(), e),
                            }
                        )
                    }
                    ArtifactStatus::Running(id) => {
                        info!("Artifact {} already running with id {}", name, id);
                        Task::none()
                    }
                    ArtifactStatus::Starting => {
                        info!("Artifact {} is starting...", name);
                        Task::none()
                    }
                    ArtifactStatus::Error(e) => {
                        info!("Found error for {}: {}. Retrying...", name, e);
                        self.state.artifact_statuses.insert(name, ArtifactStatus::Idle);
                        Task::none()
                    }
                }
            }
            Message::ArtifactStarted(name, id) => {
                self.state.artifact_statuses.insert(name, ArtifactStatus::Running(id));
                Task::none()
            }
            Message::ArtifactError(name, err) => {
                self.state.artifact_statuses.insert(name, ArtifactStatus::Error(err));
                Task::none()
            }
            Message::Tick(_now) => {
                if self.state.mode != crate::ui::state::WidgetMode::Collapsed && self.state.animation_progress.progress < 1.0 {
                    self.state.animation_progress.progress = (self.state.animation_progress.progress + 0.1).min(1.0);
                } else if self.state.mode == crate::ui::state::WidgetMode::Collapsed && self.state.animation_progress.progress > 0.0 {
                    self.state.animation_progress.progress = (self.state.animation_progress.progress - 0.1).max(0.0);
                }

                // Handle recommendations delay
                if self.state.mode != crate::ui::state::WidgetMode::Collapsed && self.state.animation_progress.progress >= 1.0 {
                    if !self.state.show_recommendations {
                        self.state.recommendations_timer += 0.05; // ~50ms per tick roughly (16ms * 3 is 48ms, I'll use 0.05 to reach 1.0 in ~20 ticks)
                        if self.state.recommendations_timer >= 1.0 {
                            self.state.show_recommendations = true;
                        }
                    }
                } else {
                    self.state.show_recommendations = false;
                    self.state.recommendations_timer = 0.0;
                }
                Task::none()
            }
            Message::WindowEvent(_id, event) => {
                match event {
                    window::Event::Focused => self.state.is_hovered = true,
                    window::Event::Unfocused => self.state.is_hovered = false,
                    _ => {}
                }
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let alpha = self.state.animation_progress.progress;
        
        let width = 48.0 + (400.0 - 48.0) * alpha;
        let is_active = self.state.mode != crate::ui::state::WidgetMode::Collapsed;

        let search_bar = components::search_bar(
            &self.state.input_text,
            width,
            alpha,
            is_active
        );

        if self.state.show_recommendations {
            let mut results_col = column![
                row![
                    text("RECOMMENDED ARTIFACTS")
                        .size(10)
                        .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
                    Space::new().width(Length::Fill),
                    components::plus_icon_button(alpha),
                ]
                .align_y(Alignment::Center)
                .padding([0, 12])
            ].spacing(4);

            if self.state.show_create_menu {
                let create_menu = container(
                    button(
                        row![
                            text("+").size(14),
                            Space::new().width(8),
                            text("Artifact").size(13),
                        ]
                        .align_y(Alignment::Center)
                    )
                    .on_press(Message::CreateArtifact)
                    .padding([8, 16])
                    .style(|_theme, status| {
                        let is_hovered = status == button::Status::Hovered;
                        button::Style {
                            background: if is_hovered {
                                Some(Color::from_rgba(0.0, 0.0, 0.0, 0.05).into())
                            } else {
                                Some(Color::from_rgba(0.0, 0.0, 0.0, 0.02).into())
                            },
                            border: iced::Border {
                                radius: 8.0.into(),
                                width: 0.0,
                                color: Color::TRANSPARENT,
                            },
                            text_color: Color::BLACK,
                            shadow: iced::Shadow::default(),
                            snap: false,
                        }
                    })
                )
                .padding([4, 12])
                .width(Length::Fill);

                results_col = results_col.push(create_menu);
            }

            let dummy_artifacts = vec![
                ("File Manager".to_string(), "Podman GUI Helper".to_string()),
                ("Terminal".to_string(), "Container Console".to_string()),
            ];

            for (title, subtitle) in dummy_artifacts {
                let status = self.state.artifact_statuses.get(&title);
                let is_setup = self.state.setup_artifacts.contains(&title);
                results_col = results_col.push(components::artifact_item(title, subtitle, is_setup, status, alpha));
            }

            let dropdown = components::artifact_card(results_col, alpha);

            container(
                column![
                    dropdown,
                    Space::new().height(12),
                    search_bar,
                ]
                .align_x(Alignment::Center)
                .spacing(12)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .align_y(Alignment::End)
            .padding(24)
            .style(move |_theme| container::Style {
                background: Some(Color::from_rgba(0.2, 0.8, 0.2, 0.02).into()), // Subtle green background tint
                ..Default::default()
            })
            .into()
        } else {
            container(search_bar)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .align_y(Alignment::End)
                .padding(24)
                .style(move |_theme| container::Style {
                    background: Some(Color::from_rgba(0.2, 0.8, 0.2, 0.02).into()),
                    ..Default::default()
                })
                .into()
        }
    }
}
