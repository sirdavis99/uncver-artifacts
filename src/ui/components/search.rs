use iced::widget::{button, container, row, TextInput, Space};
use iced::{Alignment, Color, Element, Length, Font, font};
use crate::ui::widget::Message;
use super::icons::{SEARCH_SVG, CLEAR_SVG};

pub fn search_icon_button<'a>(is_active: bool, alpha: f32) -> Element<'a, Message> {
    let svg_handle = iced::widget::svg::Handle::from_memory(SEARCH_SVG.as_bytes().to_vec());
    let icon = container(iced::widget::svg(svg_handle).width(20).height(20))
        .width(48)
        .height(48)
        .center_x(48)
        .center_y(48);

    if is_active {
        button(icon)
            .padding(0)
            .style(move |_theme, status| {
                let is_hovered = status == button::Status::Hovered;
                button::Style {
                    background: if is_hovered {
                        Some(Color::from_rgba(0.0, 0.0, 0.0, 0.03 * alpha).into())
                    } else {
                        None
                    },
                    border: iced::Border {
                        radius: 24.0.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    text_color: Color::from_rgba(0.2, 0.2, 0.2, alpha), // Darker gray for active icon
                    ..Default::default()
                }
            })
            .on_press(Message::ToggleSearch)
            .into()
    } else {
        icon.into()
    }
}

pub fn clear_button<'a>(alpha: f32) -> Element<'a, Message> {
    let svg_handle = iced::widget::svg::Handle::from_memory(CLEAR_SVG.as_bytes().to_vec());
    button(
        container(iced::widget::svg(svg_handle).width(10).height(10))
            .width(24)
            .height(24)
            .center_x(24)
            .center_y(24)
    )
    .padding(0)
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, if is_hovered { 0.1 } else { 0.06 } * alpha).into()),
            border: iced::Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            text_color: Color::from_rgba(0.3, 0.3, 0.3, alpha),
            ..Default::default()
        }
    })
    .on_press(Message::Clear)
    .into()
}

pub fn search_bar<'a>(
    input_text: &str,
    width: f32,
    alpha: f32,
    is_active: bool,
) -> Element<'a, Message> {
    let mut bar_content = row![
        search_icon_button(is_active, alpha),
    ]
    .align_y(Alignment::Center)
    .spacing(0);

    if width > 100.0 {
        bar_content = bar_content.push(
            TextInput::new("Search artifacts...", input_text)
                .on_input(Message::SearchChanged)
                .size(20) // Increased from 16
                .font(Font {
                    weight: font::Weight::Semibold,
                    ..Default::default()
                })
                .width(Length::Fill)
                .style(move |_theme, _status| {
                    iced::widget::text_input::Style {
                        background: Color::TRANSPARENT.into(),
                        placeholder: Color::from_rgba(0.5, 0.5, 0.5, alpha),
                        value: Color::from_rgba(0.1, 0.1, 0.1, alpha),
                        selection: Color::from_rgba(0.1, 0.1, 0.1, 0.2 * alpha),
                        border: iced::Border::default(),
                        icon: Color::TRANSPARENT,
                    }
                })
        );
        
        if !input_text.is_empty() {
            bar_content = bar_content.push(
                container(clear_button(alpha))
                    .width(48)
                    .height(48)
                    .center_x(48)
                    .center_y(48)
            );
        } else {
            bar_content = bar_content.push(Space::new().width(16));
        }
    }

    let bar = container(bar_content)
        .width(width)
        .height(48)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.95 * alpha).into()), // Small transparency (0.95)
            border: iced::Border {
                radius: 24.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.2, 0.8, 0.2, 0.3 * alpha), // Active green shadow
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        });

    button(bar)
        .padding(0)
        .on_press(Message::ToggleSearch)
        .style(move |_theme, status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: if is_active {
                    None // Handled by container
                } else if is_hovered {
                    Some(Color::from_rgba(1.0, 1.0, 1.0, 1.0).into())
                } else {
                    Some(Color::from_rgba(1.0, 1.0, 1.0, 0.5).into()) // Initial slight transparency (0.5)
                },
                border: iced::Border {
                    radius: 24.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: if !is_active && is_hovered {
                    iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 8.0,
                    }
                } else {
                    iced::Shadow::default()
                },
                text_color: Color::from_rgba(0.3, 0.3, 0.3, alpha), // Icon color
                ..Default::default()
            }
        })
        .into()
}
