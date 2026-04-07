use iced::widget::{button, column, container, svg, text, Row, Space};
use iced::{Alignment, Color, Element, Length, Pixels, Padding, Background, Border, Vector, Font};
use crate::ui::state::ArtifactStatus;
use crate::ui::widget::Message;
use super::icons::PLUS_SVG;

pub fn plus_icon_button<'a>(alpha: f32) -> Element<'a, Message> {
    let svg_handle = svg::Handle::from_memory(PLUS_SVG.as_bytes().to_vec());
    button(
        container(svg(svg_handle).width(12).height(12))
            .width(Pixels(24.0))
            .height(Pixels(24.0))
            .center_x(Pixels(24.0))
            .center_y(Pixels(24.0))
    )
    .padding(0)
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: if is_hovered {
                Some(Color::from_rgba(0.0, 0.0, 0.0, 0.08 * alpha).into())
            } else {
                None
            },
            border: Border {
                radius: 6.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::from_rgba(0.4, 0.4, 0.4, alpha),
            shadow: iced::Shadow::default(),
            ..Default::default()
        }
    })
    .on_press(Message::ToggleCreateMenu)
    .into()
}

pub fn artifact_item<'a>(
    title: String,
    subtitle: String,
    is_setup: bool,
    status: Option<&ArtifactStatus>,
    alpha: f32,
) -> Element<'a, Message> {
    let mut row_children = Vec::new();

    // 1. Info Selection (Title and Description)
    row_children.push(
        column![
            text(title.clone())
                .size(13)
                .font(Font {
                    family: iced::font::Family::SansSerif,
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .color(Color::from_rgba(0.1, 0.1, 0.1, alpha)),
            text(subtitle)
                .size(11)
                .font(Font::default())
                .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
        ]
        .spacing(2)
        .width(Length::Fill)
        .into()
    );

    // 2. Right Section: Indicator or Start Badge
    match status {
        Some(ArtifactStatus::Starting) => {
            row_children.push(
                container(Space::new())
                    .width(16)
                    .height(16)
                    .style(move |_| {
                        container::Style {
                            border: Border {
                                color: Color::from_rgb(0.2, 0.7, 0.2),
                                width: 2.0,
                                radius: 8.0.into(),
                            },
                            ..Default::default()
                        }
                    })
                    .into(),
            );
        }
        _ if is_setup && (status.is_none() || matches!(status, Some(ArtifactStatus::Idle))) => {
            row_children.push(
                container(
                    text("Start")
                        .size(9)
                        .font(Font::default())
                        .color(Color::WHITE)
                )
                .padding([4, 10])
                .style(move |_theme| {
                    container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.15, 0.75, 0.15, alpha))),
                        border: Border {
                            radius: 16.0.into(),
                            ..Default::default()
                        },
                        shadow: iced::Shadow {
                            color: Color::from_rgba(0.1, 0.5, 0.1, 0.3 * alpha),
                            offset: iced::Vector::new(0.0, 2.0),
                            blur_radius: 8.0,
                        },
                        ..Default::default()
                    }
                })
                .into()
            );
        }
        _ => {}
    }

    let item_content = Row::with_children(row_children)
        .align_y(Alignment::Center)
        .spacing(12);

    button(
        container(item_content)
            .padding(Padding { left: 16.0, right: 12.0, top: 12.0, bottom: 12.0 })
            .width(Length::Fill)
    )
    .width(Length::Fill)
    .padding(0)
    .on_press(Message::OpenViewModal(title))
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: if is_hovered {
                Some(Color::from_rgba(0.0, 0.0, 0.0, 0.03 * alpha).into())
            } else {
                None
            },
            border: Border {
                radius: 12.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            ..Default::default()
        }
    })
    .into()
}

pub fn artifact_card<'a>(
    content: impl Into<Element<'a, Message>>,
    alpha: f32,
) -> Element<'a, Message> {
    container(
        container(content)
            .padding(Padding { top: 4.0, bottom: 1.0, left: 4.0, right: 4.0 })
    )
    .width(Pixels(400.0))
    .height(Pixels(200.0)) // Ensure the card maintains a solid minimal aesthetic block
    .style(move |_theme| container::Style {
        background: Some(Color::from_rgba(1.0, 1.0, 1.0, alpha).into()),
        border: Border {
            radius: 16.0.into(), // Slightly refined radius to match modal
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15 * alpha),
            offset: Vector::new(0.0, 10.0), // Deep offset
            blur_radius: 32.0, // Soft shadow
        },
        ..Default::default()
    })
    .into()
}
