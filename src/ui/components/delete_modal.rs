use iced::widget::{column, row, text};
use iced::{Alignment, Color, Element, Font, Length};
use crate::ui::widget::Message;
use super::modal_button::{danger_btn, ghost_btn};
use super::modal::modal_frame;
use super::layout::{vspace, hspace};

pub fn delete_confirmation_modal<'a>(
    artifact_name: &'a str,
    alpha: f32,
    _is_loading: bool,
) -> Element<'a, Message> {
    let title = text("Remove Artifact")
        .size(18)
        .font(Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Bold,
            ..iced::Font::DEFAULT
        })
        .color(Color::from_rgba(0.08, 0.08, 0.08, alpha));

    let message = text(format!(
        "Are you sure you want to remove \"{}\"? This action cannot be undone.",
        artifact_name
    ))
    .size(14)
    .color(Color::from_rgba(0.3, 0.3, 0.3, alpha));

    let footer = row![
        ghost_btn("Cancel", Message::CancelDeleteArtifact, alpha),
        hspace(Length::Fill),
        danger_btn("Remove Permanently", Message::SubmitDeleteArtifact, alpha),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let content = column![
        title,
        vspace(16),
        message,
        vspace(32),
        footer,
    ]
    .padding(24)
    .width(Length::Fixed(360.0)); // Fixed width for confirmation dialog

    modal_frame(content, alpha)
}
