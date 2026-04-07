use iced::widget::{column, row};
use iced::{Alignment, Element, Length};
use crate::ui::widget::Message;
use super::modal_button::blue_btn;
use super::modal::{modal_frame, modal_header};
use super::layout::{vspace, hspace};
use super::inputfield::labeled_input;
use super::folder_picker::folder_picker_row;
use std::path::Path;

pub fn edit_artifact_modal<'a>(
    title: &'a str,
    description: &'a str,
    folder_path: Option<&'a Path>,
    alpha: f32,
    is_loading: bool,
) -> Element<'a, Message> {
    // ── Input Fields ─────────────────────────────────────────────────────
    let title_section = labeled_input(
        "ARTIFACT NAME",
        "Give it a name…",
        title,
        Message::CreateTitleChanged,
        alpha,
    );

    let desc_section = labeled_input(
        "DESCRIPTION",
        "What does this artifact do?",
        description,
        Message::CreateDescriptionChanged,
        alpha,
    );

    // ── Folder Picker ────────────────────────────────────────────────────
    let folder_section = folder_picker_row(
        "TARGET FOLDER",
        folder_path,
        alpha,
    );

    // ── Header ───────────────────────────────────────────────────────────
    let header = modal_header("Edit Artifact", alpha);

    // ── Actions ──────────────────────────────────────────────────────────
    // Note: Cancel moved to the X in header, or removed from footer per requirement
    let footer_row = row![
        hspace(Length::Fill),
        blue_btn("Save Changes", Message::SubmitUpdateArtifact, alpha, is_loading),
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let content = column![
        header,
        vspace(24),
        title_section,
        vspace(16),
        desc_section,
        vspace(16),
        folder_section,
        vspace(32),
        footer_row,
    ]
    .spacing(4)
    .padding(20);

    modal_frame(content, alpha)
}
