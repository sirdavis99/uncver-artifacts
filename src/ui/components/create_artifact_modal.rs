use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Background, Color, Element, Length, Padding, Font};
use iced::font::Weight;
use crate::ui::widget::Message;
use std::path::Path;

pub fn create_artifact_modal<'a>(
    title: &'a str,
    description: &'a str,
    folder_path: Option<&'a Path>,
    alpha: f32,
) -> Element<'a, Message> {
    let input_style = move |_theme: &iced::Theme, status: text_input::Status| -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgba(1.0, 1.0, 1.0, alpha)),
            border: iced::Border {
                color: match status {
                    text_input::Status::Focused { .. } => Color::from_rgba(0.2, 0.6, 1.0, alpha),
                    _ => Color::from_rgba(0.85, 0.85, 0.85, alpha),
                },
                width: 1.0,
                radius: 6.0.into(),
            },
            icon: Color::from_rgba(0.5, 0.5, 0.5, alpha),
            placeholder: Color::from_rgba(0.6, 0.6, 0.6, alpha),
            value: Color::from_rgba(0.1, 0.1, 0.1, alpha),
            selection: Color::from_rgba(0.8, 0.9, 1.0, alpha),
        }
    };

    let title_input = text_input("Artifact Title (Required)", title)
        .on_input(Message::CreateTitleChanged)
        .padding(10)
        .size(14)
        .style(input_style.clone());

    let desc_input = text_input("Description (Optional)", description)
        .on_input(Message::CreateDescriptionChanged)
        .padding(10)
        .size(13)
        .style(input_style.clone());

    let folder_text = match folder_path {
        Some(p) => p.to_string_lossy().to_string(),
        None => "No folder selected".to_string(),
    };

    let folder_btn = button(
        text("Select Folder").size(12).color(Color::from_rgba(0.2, 0.2, 0.2, alpha))
    )
    .on_press(Message::SelectCreateFolder)
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: Some(Background::Color(
                if is_hovered { Color::from_rgba(0.92, 0.92, 0.92, alpha) } else { Color::from_rgba(0.96, 0.96, 0.96, alpha) }
            )),
            border: iced::Border {
                radius: 6.0.into(),
                width: 1.0,
                color: Color::from_rgba(0.85, 0.85, 0.85, alpha),
            },
            text_color: Color::from_rgba(0.1, 0.1, 0.1, alpha),
            ..Default::default()
        }
    })
    .padding([6, 12]);

    let folder_row = row![
        folder_btn,
        text(folder_text).size(12).color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let title_is_empty = title.is_empty();
    let mut create_btn = button(
        container(text(if title_is_empty { "Missing Title" } else { "Create" }).size(14))
            .width(Length::Fill)
            .center_x(Length::Fill)
    )
    .width(Length::Fill)
    .padding(10)
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: Some(Background::Color(
                if title_is_empty {
                    Color::from_rgba(0.9, 0.9, 0.9, alpha)
                } else if is_hovered {
                    Color::from_rgba(0.15, 0.65, 0.15, alpha)
                } else {
                    Color::from_rgba(0.2, 0.8, 0.2, alpha) // Active green
                }
            )),
            border: iced::Border { radius: 6.0.into(), width: 0.0, color: Color::TRANSPARENT },
            text_color: if title_is_empty { Color::from_rgba(0.6, 0.6, 0.6, alpha) } else { Color::WHITE },
            ..Default::default()
        }
    });

    if !title_is_empty {
        create_btn = create_btn.on_press(Message::SubmitCreateArtifact);
    }

    let cancel_btn = button(
        container(text("Cancel").size(14).color(Color::from_rgba(0.8, 0.3, 0.3, alpha)))
            .width(Length::Fill)
            .center_x(Length::Fill)
    )
    .on_press(Message::CloseCreateModal)
    .width(Length::Fill)
    .padding(10)
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: Some(Background::Color(
                if is_hovered { Color::from_rgba(0.95, 0.85, 0.85, alpha) } else { Color::from_rgba(0.95, 0.95, 0.95, alpha) }
            )),
            border: iced::Border { radius: 6.0.into(), width: 1.0, color: Color::from_rgba(0.85, 0.85, 0.85, alpha) },
            ..Default::default()
        }
    });

    let main_col = column![
        text("Create Artifact")
            .size(16)
            .font(Font { weight: Weight::Bold, ..Default::default() })
            .color(Color::from_rgba(0.1, 0.1, 0.1, alpha)),
        Space::new().height(Length::Fixed(8.0)),
        title_input,
        desc_input,
        folder_row,
        Space::new().height(Length::Fixed(8.0)),
        row![cancel_btn, create_btn].spacing(8),
    ]
    .spacing(10)
    .padding(16);

    container(main_col)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, alpha))),
            border: iced::Border {
                color: Color::from_rgba(0.92, 0.92, 0.92, alpha),
                width: 1.0,
                radius: 12.0.into(),
            },
            ..container::Style::default()
        })
        .into()
}
