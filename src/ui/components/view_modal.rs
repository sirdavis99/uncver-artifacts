use iced::widget::{column, row, text};
use iced::{Alignment, Color, Element, Font, Length};
use crate::ui::widget::Message;
use super::modal_button::{primary_btn, secondary_btn, danger_btn};
use super::modal::{modal_frame, modal_header};
use super::layout::{vspace, hspace};
use std::path::Path;

fn field_label<'a>(label: &'a str, alpha: f32) -> iced::widget::Text<'a> {
    text(label)
        .size(12)
        .font(Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Bold,
            ..iced::Font::DEFAULT
        })
        .color(Color::from_rgba(0.4, 0.4, 0.4, alpha))
}

fn field_content<'a>(content: String, alpha: f32) -> iced::widget::Text<'a> {
    text(content)
        .size(14)
        .color(Color::from_rgba(0.12, 0.12, 0.12, alpha))
}

pub fn view_artifact_modal<'a>(
    title: &'a str,
    description: &'a str,
    folder_path: Option<&'a Path>,
    alpha: f32,
    is_loading: bool,
) -> Element<'a, Message> {
    let folder_str = folder_path
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "Not set".to_string());

    let header = modal_header(title, alpha);

    // ── Start Action (Centered Pill) ──────────────────────────────────────
    let start_action = column![
        primary_btn("▶  Start Artifact", Message::OpenArtifact(title.to_string()), alpha, is_loading),
    ]
    .width(Length::Fill)
    .align_x(Alignment::Center);

    // ── Description Section ──────────────────────────────────────────────
    let description_section = column![
        field_label("DESCRIPTION", alpha),
        field_content(if description.is_empty() { "No description provided." } else { description }.to_string(), alpha),
    ]
    .spacing(4);

    // ── Folder Section ───────────────────────────────────────────────────
    let folder_section = column![
        field_label("FOLDER", alpha),
        field_content(folder_str, alpha),
    ]
    .spacing(4);

    // ── Footer Actions ───────────────────────────────────────────────────
    let footer_row = row![
        danger_btn("Remove Artifact", Message::ConfirmDeleteArtifact, alpha),
        hspace(Length::Fill),
        secondary_btn("Edit Artifact", Message::SubmitEditArtifact, alpha),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let content = column![
        header,
        vspace(24),
        start_action,
        vspace(24),
        description_section,
        vspace(16),
        folder_section,
        vspace(32),
        footer_row,
    ]
    .spacing(0)
    .padding(20);

    modal_frame(content, alpha)
}
