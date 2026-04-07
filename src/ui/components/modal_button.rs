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

/// Full-width primary button (green).
pub fn primary_btn<'a>(label: &'a str, msg: Message, alpha: f32, is_loading: bool) -> Element<'a, Message> {
    if is_loading {
        return disabled_btn(format!("{}...", label), alpha);
    }

    button(
        container(text(label).size(12).font(Font { weight: Weight::Semibold, ..Font::default() }))
            .width(Length::Fill)
            .center_x(Length::Fill)
    )
    .on_press(msg)
    .width(Length::Fill)
    .padding([8, 12])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.10, 0.62, 0.10, alpha),
            _ => Color::from_rgba(0.15, 0.70, 0.15, alpha),
        })),
        border: iced::Border { radius: 8.0.into(), ..Default::default() },
        text_color: Color::WHITE,
        shadow: iced::Shadow {
            color: Color::from_rgba(0.08, 0.45, 0.08, 0.3 * alpha),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    })
    .into()
}

/// Compact secondary button (light grey, bordered, shrink width).
pub fn secondary_btn<'a>(label: &'a str, msg: Message, alpha: f32) -> Element<'a, Message> {
    button(
        container(text(label).size(11).font(Font::default()))
            .padding([0, 4])
    )
    .on_press(msg)
    .width(Length::Shrink)
    .padding([6, 12])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.87, 0.87, 0.87, alpha),
            _ => Color::from_rgba(0.93, 0.93, 0.93, alpha),
        })),
        border: iced::Border {
            radius: 6.0.into(),
            width: 1.0,
            color: Color::from_rgba(0.78, 0.78, 0.78, alpha),
        },
        text_color: Color::from_rgba(0.15, 0.15, 0.15, alpha),
        ..Default::default()
    })
    .into()
}

/// Full-width blue action button (for save/update).
pub fn blue_btn<'a>(label: &'a str, msg: Message, alpha: f32, is_loading: bool) -> Element<'a, Message> {
    if is_loading {
        return disabled_btn(format!("{}...", label), alpha);
    }

    button(
        container(text(label).size(12).font(Font { weight: Weight::Semibold, ..Font::default() }))
            .width(Length::Fill)
            .center_x(Length::Fill)
    )
    .on_press(msg)
    .width(Length::Fill)
    .padding([8, 12])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.10, 0.50, 0.88, alpha),
            _ => Color::from_rgba(0.18, 0.58, 0.96, alpha),
        })),
        border: iced::Border { radius: 8.0.into(), ..Default::default() },
        text_color: Color::WHITE,
        shadow: iced::Shadow {
            color: Color::from_rgba(0.1, 0.38, 0.8, 0.28 * alpha),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    })
    .into()
}

/// Full-width destructive button (red).
pub fn danger_btn<'a>(label: &'a str, msg: Message, alpha: f32) -> Element<'a, Message> {
    button(
        container(text(label).size(12).font(Font::default()))
            .width(Length::Fill)
            .center_x(Length::Fill)
    )
    .on_press(msg)
    .width(Length::Fill)
    .padding([8, 12])
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered => Color::from_rgba(0.70, 0.12, 0.12, alpha),
            _ => Color::from_rgba(0.80, 0.18, 0.18, alpha),
        })),
        border: iced::Border { radius: 8.0.into(), ..Default::default() },
        text_color: Color::WHITE,
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
