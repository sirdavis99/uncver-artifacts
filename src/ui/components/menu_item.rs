use iced::widget::{button, row, text, Space, svg};
use iced::{Alignment, Background, Color, Element, Padding, Font};
use crate::ui::widget::Message;

pub fn menu_button<'a>(
    label: String,
    icon_svg: &'a str,
    on_press: Option<Message>,
    alpha: f32,
) -> Element<'a, Message> {
    let mut bg_color = Color::from_rgba(0.95, 0.95, 0.95, alpha); // Lighter background
    let mut text_color = Color::from_rgba(0.2, 0.2, 0.2, alpha); // Darker text
    let mut is_disabled = false;

    if on_press.is_none() {
        // Disabled visually
        bg_color = Color::from_rgba(0.98, 0.98, 0.98, alpha * 0.8);
        text_color = Color::from_rgba(0.7, 0.7, 0.7, alpha * 0.8);
        is_disabled = true;
    }
    
    let svg_handle = svg::Handle::from_memory(icon_svg.as_bytes().to_vec());
    let svg_widget = svg(svg_handle)
        .width(iced::Pixels(14.0))
        .height(iced::Pixels(14.0))
        .style(move |_theme, _status| iced::widget::svg::Style {
            color: Some(text_color),
        });

    let mut btn = button(
        row![
            svg_widget,
            Space::new().width(4),
            text(label).size(11).font(Font::default()).color(text_color),
        ]
        .align_y(Alignment::Center)
    )
    .padding(Padding { top: 6.0, bottom: 6.0, left: 10.0, right: 10.0 })
    .style(move |_theme, status| button::Style {
        background: Some(Background::Color(match status {
            button::Status::Hovered if !is_disabled => Color::from_rgba(0.9, 0.9, 0.9, alpha),
            _ => bg_color,
        })),
        border: iced::Border {
            radius: 6.0.into(), // Tighter border radius 
            color: Color::TRANSPARENT,
            width: 0.0,
        },
        text_color,
        shadow: iced::Shadow::default(),
        ..Default::default()
    });

    if !is_disabled {
        if let Some(msg) = on_press {
            btn = btn.on_press(msg);
        }
    }

    btn.into()
}
