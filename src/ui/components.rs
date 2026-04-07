use iced::widget::{button, container, row, svg, text, Space};
use iced::{Alignment, Color, Element, Length, Theme};

use crate::ui::widget::Message;

pub const SEARCH_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>"#;

/// A reusable component for the individual items in the search results card.
pub fn result_item<'a>(title: &'a str, subtitle: &'a str, alpha: f32) -> Element<'a, Message> {
    let svg_handle = svg::Handle::from_memory(SEARCH_SVG.as_bytes().to_vec());
    
    container(
        button(
            iced::widget::Row::new()
                .push(
                    container(svg(svg_handle).width(16).height(16))
                        .padding(8)
                        .style(move |_theme| container::Style {
                            background: Some(Color::from_rgba(0.95, 0.95, 0.95, alpha).into()),
                            border: iced::Border { radius: 8.0.into(), ..Default::default() },
                            ..Default::default()
                        })
                )
                .push(Space::new().width(12.0))
                .push(
                    iced::widget::Column::new()
                        .push(text(title).size(14).color(Color::from_rgba(0.1, 0.1, 0.1, alpha)))
                        .push(text(subtitle).size(11).color(Color::from_rgba(0.5, 0.5, 0.5, alpha)))
                )
                .align_y(Alignment::Center)
        )
        .padding(8)
        .width(Length::Fill)
        .style(move |_theme, status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: if is_hovered {
                    Some(Color::from_rgba(0.0, 0.0, 0.0, 0.05 * alpha).into())
                } else {
                    None
                },
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                text_color: Color::BLACK,
                ..Default::default()
            }
        })
        .on_press(Message::OpenArtifact(title.to_string()))
    )
    .padding(iced::Padding { left: 8.0, top: 2.0, right: 8.0, bottom: 2.0 })
    .width(Length::Fill)
    .into()
}

/// A reusable component for the search icon button.
pub fn search_icon_button<'a>() -> Element<'a, Message> {
    let svg_handle = svg::Handle::from_memory(SEARCH_SVG.as_bytes().to_vec());
    button(
        container(svg(svg_handle).width(24).height(24))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
    )
    .on_press(Message::IconClick)
    .padding(0)
    .style(|_theme: &Theme, _status: button::Status| button::Style {
        background: None,
        ..Default::default()
    })
    .into()
}

/// A reusable 'X' clear button for the search input.
pub fn clear_button<'a>(alpha: f32) -> Element<'a, Message> {
    button(
        container(text("✕").size(14).color(Color::from_rgba(0.4, 0.4, 0.4, alpha)))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
    )
    .on_press(Message::Clear)
    .width(26)
    .height(26)
    .padding(0)
    .style(move |_theme, _| button::Style {
        background: Some(Color::from_rgba(0.92, 0.92, 0.92, alpha).into()),
        border: iced::Border { radius: 13.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}
