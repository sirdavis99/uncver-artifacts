use iced::widget::{column, container, row, text, Space, Column};
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

        let content = if self.state.show_recommendations {
            self.render_recommendations(search_bar, alpha)
        } else {
            column![search_bar].into()
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .align_y(Alignment::End)
            .padding(24)
            .into()
    }

    fn render_recommendations<'a>(&self, search_bar: Element<'a, Message>, alpha: f32) -> Element<'a, Message> {
        let title = if self.state.input_text.is_empty() {
            "RECOMMENDED ARTIFACTS"
        } else {
            "SEARCH RESULTS"
        };

        let mut results_col = Column::new()
            .spacing(2)
            .width(400);

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

        let recommended_artifacts = components::artifact_card(results_col, alpha);

        column![
            recommended_artifacts,
            search_bar,
        ]
        .align_x(Alignment::Center)
        .spacing(10)
        .into()
    }
}
