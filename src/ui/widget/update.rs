use iced::Task;
use tracing::info;
use crate::ui::state::{ArtifactStatus, WidgetMode};
use crate::ui::widget::{SearchWidget, Message};
use crate::artifacts::ArtifactConfig;

impl SearchWidget {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchChanged(val) => {
                self.state.input_text = val;
                self.state.save_persistent();
                Task::none()
            }
            Message::ToggleSearch => {
                if self.state.mode == WidgetMode::Collapsed {
                    self.state.mode = WidgetMode::SearchMode;
                } else {
                    self.state.mode = WidgetMode::Collapsed;
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
            Message::OpenCreateModal => {
                self.state.show_create_menu = false;
                self.state.show_create_modal = true;
                self.state.reset_create_form();
                Task::none()
            }
            Message::CloseCreateModal => {
                self.state.show_create_modal = false;
                Task::none()
            }
            Message::CreateTitleChanged(val) => {
                self.state.create_form_title = val;
                Task::none()
            }
            Message::CreateDescriptionChanged(val) => {
                self.state.create_form_description = val;
                Task::none()
            }
            Message::SelectCreateFolder => {
                Task::perform(
                    async {
                        tokio::task::spawn_blocking(|| {
                            rfd::FileDialog::new().pick_folder()
                        }).await.unwrap_or(None)
                    },
                    Message::FolderSelected
                )
            }
            Message::FolderSelected(path) => {
                if let Some(p) = path {
                    self.state.create_form_folder = Some(p);
                }
                Task::none()
            }
            Message::SubmitCreateArtifact => {
                if self.state.create_form_title.is_empty() {
                    return Task::none();
                }
                
                let title = self.state.create_form_title.clone();
                let desc = self.state.create_form_description.clone();
                let folder = self.state.create_form_folder.clone();
                
                let manager = self.artifacts.clone();
                self.state.show_create_modal = false;
                self.state.reset_create_form();
                
                Task::perform(
                    async move {
                        let config = ArtifactConfig {
                            name: title.clone(),
                            description: Some(desc),
                            url: None,
                            local_path: folder.map(|p| p.to_string_lossy().to_string()),
                            container_image: Some("ghcr.io/podman/hello:latest".to_string()),
                        };
                        if manager.create_artifact(&config).is_ok() {
                            Some(title)
                        } else {
                            None
                        }
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
            Message::OpenViewModal(name) => {
                if let Some(artifact) = self.state.artifacts.iter().find(|a| a.name == name) {
                    self.state.selected_artifact = Some(name.clone());
                    self.state.create_form_title = artifact.name.clone();
                    self.state.create_form_description = artifact.description.clone().unwrap_or_default();
                    self.state.create_form_folder = artifact.local_path.as_ref().map(|p| std::path::PathBuf::from(p));
                    self.state.is_viewing = true;
                    self.state.show_create_modal = true;
                }
                Task::none()
            }
            Message::OpenEditModal(name) => {
                if let Some(artifact) = self.state.artifacts.iter().find(|a| a.name == name) {
                    self.state.selected_artifact = Some(name.clone());
                    self.state.create_form_title = artifact.name.clone();
                    self.state.create_form_description = artifact.description.clone().unwrap_or_default();
                    self.state.create_form_folder = artifact.local_path.as_ref().map(|p| std::path::PathBuf::from(p));
                    self.state.is_viewing = false;
                    self.state.show_create_modal = true;
                }
                Task::none()
            }
            Message::SubmitEditArtifact => {
                self.state.is_viewing = false;
                Task::none()
            }
            Message::SubmitUpdateArtifact => {
                if let Some(old_name) = self.state.selected_artifact.clone() {
                    let title = self.state.create_form_title.clone();
                    let desc = self.state.create_form_description.clone();
                    let folder = self.state.create_form_folder.clone();
                    let manager = self.artifacts.clone();
                    
                    self.state.show_create_modal = false;
                    self.state.selected_artifact = None;
                    self.state.reset_create_form();
                    
                    Task::perform(
                        async move {
                            let config = ArtifactConfig {
                                name: title.clone(),
                                description: Some(desc),
                                url: None,
                                local_path: folder.map(|p| p.to_string_lossy().to_string()),
                                container_image: Some("ghcr.io/podman/hello:latest".to_string()),
                            };
                            manager.update_artifact(&old_name, &config).is_ok()
                        },
                        |_| Message::RefreshArtifacts
                    )
                } else {
                    Task::none()
                }
            }
            Message::SubmitDeleteArtifact => {
                if let Some(name) = self.state.selected_artifact.clone() {
                    let manager = self.artifacts.clone();
                    self.state.show_create_modal = false;
                    self.state.selected_artifact = None;
                    self.state.reset_create_form();
                    
                    Task::perform(
                        async move {
                            manager.delete_artifact(&name).is_ok()
                        },
                        |_| Message::RefreshArtifacts
                    )
                } else {
                    Task::none()
                }
            }
            Message::ArtifactCreated(name) => {
                info!("Artifact {} created!", name);
                self.handle_refresh_artifacts()
            }
            Message::ArtifactUpdated(path) => {
                info!("Artifact file updated: {:?}", path);
                Task::perform(
                    async move {
                        crate::artifacts::builder::build_from_config(path).await;
                    },
                    |_| Message::RefreshArtifacts
                )
            }
            Message::OpenArtifact(name) => self.handle_open_artifact(name),
            Message::ArtifactStarted(name, id) => {
                self.state.artifact_statuses.insert(name, ArtifactStatus::Running(id));
                Task::none()
            }
            Message::ArtifactError(name, err) => {
                self.state.artifact_statuses.insert(name, ArtifactStatus::Error(err));
                Task::none()
            }
            Message::ArtifactsLoaded(artifacts) => {
                self.state.artifacts = artifacts;
                self.state.is_loading = false;
                Task::none()
            }
            Message::RefreshArtifacts => self.handle_refresh_artifacts(),
            Message::Tick(_now) => {
                let mut tasks = vec![self.handle_tick()];
                if self.state.is_loading {
                    tasks.push(self.handle_refresh_artifacts());
                }
                Task::batch(tasks)
            },
            Message::WindowEvent(_id, event) => {
                match event {
                    iced::window::Event::Focused => self.state.is_hovered = true,
                    iced::window::Event::Unfocused => self.state.is_hovered = false,
                    _ => {}
                }
                Task::none()
            }
            Message::None => Task::none(),
        }
    }

    fn handle_refresh_artifacts(&self) -> Task<Message> {
        let manager = self.artifacts.clone();
        Task::perform(
            async move {
                manager.list_artifacts().await.unwrap_or_default()
            },
            Message::ArtifactsLoaded
        )
    }



    fn handle_open_artifact(&mut self, name: String) -> Task<Message> {
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
            ArtifactStatus::Stopping => {
                info!("Artifact {} is stopping...", name);
                Task::none()
            }
            ArtifactStatus::Error(e) => {
                info!("Found error for {}: {}. Retrying...", name, e);
                self.state.artifact_statuses.insert(name, ArtifactStatus::Idle);
                Task::none()
            }
        }
    }

    fn handle_tick(&mut self) -> Task<Message> {
        if self.state.mode != WidgetMode::Collapsed {
            // Faster fade in (0.1 per tick instead of 0.05)
            self.state.animation_progress.progress = (self.state.animation_progress.progress + 0.1).min(1.0);
            
            if self.state.animation_progress.progress >= 1.0 && !self.state.show_recommendations {
                self.state.recommendations_timer += 1.0;
                // Faster show delay (20 ticks instead of 100)
                if self.state.recommendations_timer >= 20.0 {
                    self.state.show_recommendations = true;
                }
            }
        } else {
            // Faster fade out
            self.state.animation_progress.progress = (self.state.animation_progress.progress - 0.1).max(0.0);
            self.state.show_recommendations = false;
            self.state.recommendations_timer = 0.0;
        }
        Task::none()
    }
}
