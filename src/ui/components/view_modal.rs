use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Background, Color, Element, Font, Length};
use iced::font::Weight;
use crate::ui::widget::Message;
use super::modal_button::{secondary_btn, disabled_btn};
use super::modal::{modal_frame, modal_header};
use std::path::Path;

fn field_label<'a>(label: &'a str, alpha: f32) -> iced::widget::Text<'a> {
    text(label)
        .size(12)
        .font(Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Semibold,
            ..iced::Font::DEFAULT
        })
        .color(Color::from_rgba(0.45, 0.45, 0.45, alpha))
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

    // ── Header ───────────────────────────────────────────────────────────
    let header = modal_header(title, alpha);

    // ── Description ────────────────────────────────────────────────────────
    let desc_section = column![
        field_label("DESCRIPTION", alpha),
        text(if description.is_empty() { "No description provided." } else { description })
            .size(14)
            .font(Font::default())
            .color(Color::from_rgba(0.2, 0.2, 0.2, alpha)),
    ]
    .spacing(4);

    // ── Folder Metadata ──────────────────────────────────────────────────
    let folder_section = column![
        field_label("FOLDER", alpha),
        text(folder_str)
            .size(13)
            .font(Font::default())
            .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
    ]
    .spacing(4);

    // ── Start Action (Pill Button) ─────────────────────────────────────────
    let start_button: Element<Message> = if is_loading {
        disabled_btn("Starting...", alpha)
    } else {
        button(
            container(
                row![
                    text("▶").size(10),
                    text("Start Artifact")
                        .size(12)
                        .font(Font { weight: Weight::Semibold, ..Font::default() }),
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .padding([0, 10])
        )
        .on_press(Message::OpenArtifact(title.to_string()))
        .padding([8, 16])
        .style(move |_theme, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgba(0.12, 0.65, 0.12, alpha),
                _ => Color::from_rgba(0.15, 0.70, 0.15, alpha),
            })),
            border: iced::Border { radius: 24.0.into(), ..Default::default() },
            text_color: Color::WHITE,
            shadow: iced::Shadow {
                color: Color::from_rgba(0.1, 0.5, 0.1, 0.25 * alpha),
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .into()
    };

    let start_area = container(start_button)
        .width(Length::Fill)
        .center_x(Length::Fill);

    // ── Footer ───────────────────────────────────────────────────────────
    let footer = row![
        Space::new().width(Length::Fill),
        secondary_btn("Edit Artifact", Message::SubmitEditArtifact, alpha),
    ]
    .width(Length::Fill);

    let content = column![
        header,
        Space::new().height(20),
        desc_section,
        Space::new().height(16),
        folder_section,
        Space::new().height(32),
        start_area,
        Space::new().height(20),
        footer,
    ]
    .padding(20)
    .spacing(0);

    modal_frame(content, alpha)
}
