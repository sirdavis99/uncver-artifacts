use iced::widget::{button, container, text};
use iced::{Background, Color, Element, Font, Length};
use iced::font::Weight;
use crate::ui::widget::Message;

/// Circular ✕ icon button for the modal header.
pub fn close_icon_btn<'a>(msg: Message, alpha: f32) -> Element<'a, Message> {
    let svg_handle = iced::widget::svg::Handle::from_memory(crate::ui::components::icons::CLEAR_SVG.as_bytes().to_vec());
    button(
        container(iced::widget::svg(svg_handle).width(10).height(10))
            .width(24)
            .height(24)
            .center_x(24)
            .center_y(24)
    )
    .padding(0)
    .on_press(msg)
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(
            if status == button::Status::Hovered {
                Color::from_rgba(0.0, 0.0, 0.0, 0.10 * alpha)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.06 * alpha)
            }
        )),
        border: iced::Border { radius: 12.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}

/// Primary action button (green). Default width is Shrink to allow flexible overrides.
pub fn primary_btn<'a>(label: &'a str, msg: Message, alpha: f32, is_loading: bool) -> Element<'a, Message> {
    if is_loading {
        return disabled_btn(format!("{}...", label), alpha);
    }

    button(
        container(text(label).size(13).font(Font { weight: Weight::Bold, ..Font::default() }))
            .center_x(Length::Shrink)
    )
    .on_press(msg)
    .padding([10, 24])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.12, 0.65, 0.12, alpha),
            _ => Color::from_rgba(0.15, 0.70, 0.15, alpha),
        })),
        border: iced::Border { radius: 10.0.into(), ..Default::default() },
        text_color: Color::WHITE,
        shadow: iced::Shadow {
            color: Color::from_rgba(0.12, 0.55, 0.12, 0.28 * alpha),
            offset: iced::Vector::new(0.0, 3.0),
            blur_radius: 10.0,
        },
        ..Default::default()
    })
    .into()
}

/// Compact secondary button (light grey, bordered, shrink width).
pub fn secondary_btn<'a>(label: &'a str, msg: Message, alpha: f32) -> Element<'a, Message> {
    button(
        container(text(label).size(12).font(Font { weight: Weight::Semibold, ..Font::default() }))
            .padding([0, 4])
    )
    .on_press(msg)
    .width(Length::Shrink)
    .padding([8, 16])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.90, 0.90, 0.90, alpha),
            _ => Color::from_rgba(0.95, 0.95, 0.95, alpha),
        })),
        border: iced::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: Color::from_rgba(0.8, 0.8, 0.8, alpha),
        },
        text_color: Color::from_rgba(0.2, 0.2, 0.2, alpha),
        ..Default::default()
    })
    .into()
}

/// Action button (blue).
pub fn blue_btn<'a>(label: &'a str, msg: Message, alpha: f32, is_loading: bool) -> Element<'a, Message> {
    if is_loading {
        return disabled_btn(format!("{}...", label), alpha);
    }

    button(
        container(text(label).size(13).font(Font { weight: Weight::Bold, ..Font::default() }))
            .center_x(Length::Shrink)
    )
    .on_press(msg)
    .padding([10, 24])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.12, 0.52, 0.92, alpha),
            _ => Color::from_rgba(0.18, 0.58, 0.96, alpha),
        })),
        border: iced::Border { radius: 10.0.into(), ..Default::default() },
        text_color: Color::WHITE,
        shadow: iced::Shadow {
            color: Color::from_rgba(0.1, 0.42, 0.85, 0.28 * alpha),
            offset: iced::Vector::new(0.0, 3.0),
            blur_radius: 10.0,
        },
        ..Default::default()
    })
    .into()
}

/// Destructive button (red).
pub fn danger_btn<'a>(label: &'a str, msg: Message, alpha: f32) -> Element<'a, Message> {
    button(
        container(text(label).size(13).font(Font { weight: Weight::Bold, ..Font::default() }))
            .center_x(Length::Shrink)
    )
    .on_press(msg)
    .padding([10, 24])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.72, 0.15, 0.15, alpha),
            _ => Color::from_rgba(0.82, 0.20, 0.20, alpha),
        })),
        border: iced::Border { radius: 10.0.into(), ..Default::default() },
        text_color: Color::WHITE,
        shadow: iced::Shadow {
            color: Color::from_rgba(0.65, 0.12, 0.12, 0.25 * alpha),
            offset: iced::Vector::new(0.0, 3.0),
            blur_radius: 10.0,
        },
        ..Default::default()
    })
    .into()
}

/// Compact ghost close/cancel button (no background, shrink width).
pub fn ghost_btn<'a>(label: &'a str, msg: Message, alpha: f32) -> Element<'a, Message> {
    button(
        container(text(label).size(12).font(Font::default()))
    )
    .on_press(msg)
    .width(Length::Shrink)
    .padding([7, 12])
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: if is_hovered {
                Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.04 * alpha)))
            } else {
                None
            },
            border: iced::Border { 
                radius: 8.0.into(), 
                ..Default::default() 
            },
            text_color: Color::from_rgba(0.45, 0.45, 0.45, alpha),
            ..Default::default()
        }
    })
    .into()
}

/// Disabled placeholder (no press action).
pub fn disabled_btn<'a>(label: impl Into<String>, alpha: f32) -> Element<'a, Message> {
    container(
        container(text(label.into()).size(12).font(Font::default()))
            .width(Length::Fill)
            .center_x(Length::Fill)
    )
    .width(Length::Fill)
    .padding([8, 12])
    .style(move |_| container::Style {
        background: Some(Background::Color(Color::from_rgba(0.90, 0.90, 0.90, alpha))),
        border: iced::Border { radius: 8.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}
