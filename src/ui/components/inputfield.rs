use iced::widget::{column, text, text_input};
use iced::{Background, Color, Font};
use crate::ui::widget::Message;

pub fn input_style(alpha: f32) -> impl Fn(&iced::Theme, text_input::Status) -> text_input::Style + 'static {
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

pub fn field_label<'a>(label: &'a str, alpha: f32) -> iced::widget::Text<'a> {
    text(label)
        .size(12)
        .font(Font {
            family: iced::font::Family::SansSerif,
            weight: iced::font::Weight::Semibold,
            ..iced::Font::DEFAULT
        })
        .color(Color::from_rgba(0.52, 0.52, 0.52, alpha))
}

pub fn labeled_input<'a>(
    label: &'a str,
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    alpha: f32,
) -> iced::widget::Column<'a, Message> {
    column![
        field_label(label, alpha),
        text_input(placeholder, value)
            .on_input(on_input)
            .padding([8, 12])
            .size(13)
            .font(Font::default())
            .style(input_style(alpha)),
    ]
    .spacing(8)
}
