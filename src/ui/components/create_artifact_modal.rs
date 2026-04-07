use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Background, Color, Element, Length, Font};
use iced::font::Weight;
use crate::ui::widget::Message;
use std::path::Path;

pub fn artifact_modal<'a>(
    title: &'a str,
    description: &'a str,
    folder_path: Option<&'a Path>,
    alpha: f32,
    is_editing: bool,
    is_viewing: bool,
) -> Element<'a, Message> {
    if is_viewing {
        // VIEW MODE: Single source of truth for details
        let details_col = column![
            text(title)
                .size(24)
                .font(Font { weight: Weight::Bold, ..Font::default() })
                .color(Color::from_rgba(0.1, 0.1, 0.1, alpha)),
            text(if description.is_empty() { "No description provided." } else { description })
                .size(14)
                .font(Font::default())
                .color(Color::from_rgba(0.4, 0.4, 0.4, alpha)),
            Space::new().height(12),
            row![
                text("Location: ")
                    .size(12)
                    .font(Font { weight: Weight::Bold, ..Font::default() })
                    .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
                text(folder_path.map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| "Unknown".to_string()))
                    .size(12)
                    .font(Font::default())
                    .color(Color::from_rgba(0.6, 0.6, 0.6, alpha)),
            ]
            .spacing(4),
        ]
        .spacing(8);

        let start_btn = button(
            container(text("Start Artifact").size(14).font(Font { weight: Weight::Bold, ..Font::default() }))
                .width(Length::Fill).center_x(Length::Fill)
        )
        .on_press(Message::OpenArtifact(title.to_string()))
        .width(Length::Fill)
        .padding(12)
        .style(move |_theme, status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: Some(Background::Color(if is_hovered { Color::from_rgba(0.1, 0.6, 0.1, alpha) } else { Color::from_rgba(0.15, 0.7, 0.15, alpha) })),
                border: iced::Border { radius: 8.0.into(), ..Default::default() },
                text_color: Color::WHITE,
                ..Default::default()
            }
        });

        let edit_btn = button(
            container(text("Edit Settings").size(14).font(Font::default()))
                .width(Length::Fill).center_x(Length::Fill)
        )
        .on_press(Message::SubmitEditArtifact)
        .width(Length::Fill)
        .padding(12)
        .style(move |_theme, status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: Some(Background::Color(if is_hovered { Color::from_rgba(0.9, 0.9, 0.9, alpha) } else { Color::from_rgba(0.95, 0.95, 0.95, alpha) })),
                border: iced::Border { radius: 8.0.into(), width: 1.0, color: Color::from_rgba(0.8, 0.8, 0.8, alpha) },
                text_color: Color::from_rgba(0.2, 0.2, 0.2, alpha),
                ..Default::default()
            }
        });

        let cancel_btn = button(
            container(text("Close").size(14).font(Font::default()))
                .width(Length::Fill).center_x(Length::Fill)
        )
        .on_press(Message::CloseCreateModal)
        .width(Length::Fill)
        .padding(12)
        .style(move |_theme, status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: None,
                text_color: Color::from_rgba(0.5, 0.5, 0.5, alpha),
                ..Default::default()
            }
        });

        let content = column![
            details_col,
            Space::new().height(24),
            start_btn,
            row![edit_btn, cancel_btn].spacing(8),
        ]
        .padding(24);

        return super::modal::modal_frame(content, alpha);
    }

    // CREATE/EDIT MODE (Modified from original)
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
        .font(Font::default())
        .style(input_style.clone());

    let desc_input = text_input("Description (Optional)", description)
        .on_input(Message::CreateDescriptionChanged)
        .padding(10)
        .size(13)
        .font(Font::default())
        .style(input_style.clone());

    let folder_text = match folder_path {
        Some(p) => p.to_string_lossy().to_string(),
        None => "No folder selected".to_string(),
    };

    let folder_btn = button(
        text("Select Folder")
            .size(12)
            .font(Font::default())
            .color(Color::from_rgba(0.2, 0.2, 0.2, alpha))
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
        text(folder_text)
            .size(12)
            .font(Font::default())
            .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let title_is_empty = title.is_empty();
    let action_text = if is_editing { "Update" } else { "Create" };
    
    let mut action_btn = button(
        container(
            text(if title_is_empty { "Missing Title" } else { action_text })
                .size(14)
                .font(Font::default())
        )
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
                    Color::from_rgba(0.2, 0.8, 0.2, alpha)
                }
            )),
            border: iced::Border { radius: 6.0.into(), width: 0.0, color: Color::TRANSPARENT },
            text_color: if title_is_empty { Color::from_rgba(0.6, 0.6, 0.6, alpha) } else { Color::WHITE },
            ..Default::default()
        }
    });

    if !title_is_empty {
        action_btn = action_btn.on_press(if is_editing { Message::SubmitUpdateArtifact } else { Message::SubmitCreateArtifact });
    }

    let cancel_btn = button(
        container(
            text("Cancel")
                .size(14)
                .font(Font::default())
                .color(Color::from_rgba(0.6, 0.3, 0.3, alpha))
        )
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

    let mut footer_children = vec![cancel_btn.into(), action_btn.into()];

    if is_editing {
        let remove_btn = button(
            container(
                text("Remove")
                    .size(14)
                    .font(Font::default())
                    .color(Color::WHITE)
            )
                .width(Length::Fill)
                .center_x(Length::Fill)
        )
        .on_press(Message::SubmitDeleteArtifact)
        .width(Length::Fill)
        .padding(10)
        .style(move |_theme, status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: Some(Background::Color(
                    if is_hovered { Color::from_rgba(0.75, 0.15, 0.15, alpha) } else { Color::from_rgba(0.85, 0.2, 0.2, alpha) }
                )),
                border: iced::Border { radius: 6.0.into(), width: 0.0, color: Color::TRANSPARENT },
                ..Default::default()
            }
        });
        footer_children.insert(0, remove_btn.into());
    }

    let footer = row(footer_children).spacing(8);

    let main_col = column![
        text(if is_editing { "Artifact Settings" } else { "Create Artifact" })
            .size(18)
            .font(Font { weight: Weight::Bold, ..Font::default() })
            .color(Color::from_rgba(0.1, 0.1, 0.1, alpha)),
        Space::new().height(Length::Fixed(8.0)),
        title_input,
        desc_input,
        folder_row,
        Space::new().height(Length::Fixed(8.0)),
        footer,
    ]
    .spacing(10)
    .padding(16);

    super::modal::modal_frame(main_col, alpha)
}
