use iced::widget::{column, container, row, text, Space, Column, stack};
use iced::{Alignment, Color, Element, Length, Pixels};
use crate::ui::components;
use crate::ui::state::WidgetMode;
use crate::ui::widget::{SearchWidget, Message};

impl SearchWidget {
    pub fn view(&self) -> Element<'_, Message> {
        let alpha = self.state.animation_progress.progress;
        let width = 48.0 + (400.0 - 48.0) * alpha;
        let is_active = self.state.mode != WidgetMode::Collapsed;

        let search_bar = components::search_bar(
            &self.state.input_text,
            width,
            alpha,
            is_active
        );

        let mut results_col = Column::new()
            .spacing(2)
            .width(400);
        let title = if self.state.input_text.is_empty() {
            "RECOMMENDED ARTIFACTS"
        } else {
            "SEARCH RESULTS"
        };

        results_col = results_col.push(
            row![
                text(title)
                    .size(10)
                    .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
                Space::new().width(Length::Fill),
                components::plus_icon_button(alpha),
            ]
            .align_y(Alignment::Center)
            .padding([0, 12])
        );

        if self.state.show_create_menu {
            results_col = results_col.push(components::create_artifact_menu(alpha));
        }

        if self.state.artifacts.is_empty() {
            results_col = results_col.push(
                container(
                    text("No artifacts found")
                        .size(13)
                        .color(Color::from_rgba(0.5, 0.5, 0.5, alpha))
                )
                .width(Length::Fill)
                .height(Pixels(160.0)) // Better middle ground for "well visible"
                .center_x(Length::Fill)
                .center_y(Length::Fill)
            );
        } else {
            for artifact in &self.state.artifacts {
                let status = self.state.artifact_statuses.get(&artifact.name).cloned();
                let is_setup = true; 
                results_col = results_col.push(components::artifact_item(
                    artifact.name.clone(),
                    artifact.description.clone().unwrap_or_else(|| "Artifact".to_string()),
                    is_setup,
                    status.as_ref(),
                    alpha
                ));
            }
        }

        let content: Element<'_, Message> = if self.state.show_recommendations {
            let recommended_artifacts = components::artifact_card(results_col, alpha);
            column![
                recommended_artifacts,
                search_bar,
            ]
            .align_x(Alignment::Center)
            .spacing(10)
            .into()
        } else {
            column![search_bar].into()
        };

        let main_view = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .align_y(Alignment::End)
            .padding(24);

        if self.state.show_create_modal {
            let modal = components::create_artifact_modal(
                &self.state.create_form_title,
                &self.state.create_form_description,
                self.state.create_form_folder.as_deref(),
                alpha,
            );

            let overlay = container(modal)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(move |_theme| container::Style {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..Default::default()
                });

            stack![main_view, overlay].into()
        } else {
            main_view.into()
        }
    }
}
