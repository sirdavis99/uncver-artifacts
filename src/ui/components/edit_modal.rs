use iced::widget::{button, column, row, text, text_input, Space};
use iced::{Alignment, Background, Color, Element, Font, Length};
use iced::font::Weight;
use crate::ui::widget::Message;
use super::modal_button::{blue_btn, danger_btn, ghost_btn};
use super::modal::{modal_frame, modal_header};
use std::path::Path;

fn input_style(alpha: f32) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style + 'static {
    move |_theme: &iced::Theme, status: text_input::Status| text_input::Style {
        background: Background::Color(Color::from_rgba(0.96, 0.96, 0.96, alpha)),
        border: iced::Border {
            color: match status {
                text_input::Status::Focused { .. } => Color::from_rgba(0.2, 0.55, 0.95, alpha),
                text_input::Status::Hovered => Color::from_rgba(0.68, 0.68, 0.68, alpha),
                _ => Color::from_rgba(0.80, 0.80, 0.80, alpha),
            },
            width: 1.0,
            radius: 8.0.into(),
        },
        icon: Color::from_rgba(0.5, 0.5, 0.5, alpha),
        placeholder: Color::from_rgba(0.60, 0.60, 0.60, alpha),
        value: Color::from_rgba(0.08, 0.08, 0.08, alpha),
        selection: Color::from_rgba(0.75, 0.88, 1.0, alpha),
    }
}

fn field_label<'a>(label: &'a str, alpha: f32) -> iced::widget::Text<'a> {
    text(label)
        .size(12)
        .font(Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Semibold,
            ..iced::Font::DEFAULT
        })
        .color(Color::from_rgba(0.52, 0.52, 0.52, alpha))
}

pub fn edit_artifact_modal<'a>(
    title: &'a str,
    description: &'a str,
    folder_path: Option<&'a Path>,
    alpha: f32,
    is_loading: bool,
) -> Element<'a, Message> {
    let title_input = text_input("Give it a name…", title)
        .on_input(Message::CreateTitleChanged)
        .padding([8, 12])
        .size(13)
        .font(Font::default())
        .style(input_style(alpha));

    let desc_input = text_input("What does this artifact do?", description)
        .on_input(Message::CreateDescriptionChanged)
        .padding([8, 12])
        .size(13)
        .font(Font::default())
        .style(input_style(alpha));

    let folder_str = folder_path
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "No folder selected".to_string());

    let folder_row = row![
        button(
            text("Change Destination").size(11).font(Font::default())
                .color(Color::from_rgba(0.25, 0.25, 0.25, alpha))
        )
        .on_press(Message::SelectCreateFolder)
        .padding([5, 10])
        .style(move |_theme, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgba(0.86, 0.86, 0.86, alpha),
                _ => Color::from_rgba(0.92, 0.92, 0.92, alpha),
            })),
            border: iced::Border { 
                radius: 6.0.into(), 
                width: 1.0, 
                color: Color::from_rgba(0.78, 0.78, 0.78, alpha) 
            },
            text_color: Color::from_rgba(0.15, 0.15, 0.15, alpha),
            ..Default::default()
        }),
        text(folder_str).size(11).font(Font::default())
            .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    // ── Header ───────────────────────────────────────────────────────────
    let header = modal_header("Edit Artifact", alpha);

    // ── Actions ──────────────────────────────────────────────────────────
    let footer_row = row![
        ghost_btn("Cancel", Message::CloseCreateModal, alpha),
        Space::new().width(Length::Fill),
        blue_btn("Save Changes", Message::SubmitUpdateArtifact, alpha, is_loading),
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let content = column![
        header,
        Space::new().height(16),
        field_label("ARTIFACT NAME", alpha),
        title_input,
        Space::new().height(10),
        field_label("DESCRIPTION", alpha),
        desc_input,
        Space::new().height(10),
        field_label("TARGET FOLDER", alpha),
        folder_row,
        Space::new().height(20),
        footer_row,
        Space::new().height(8),
        danger_btn("Remove Artifact", Message::SubmitDeleteArtifact, alpha),
    ]
    .spacing(4)
    .padding(18);

    modal_frame(content, alpha)
}
