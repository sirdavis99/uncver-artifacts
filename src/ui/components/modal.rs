use iced::widget::container;
use iced::{Color, Element, Length, Background};

pub fn modal_frame<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    alpha: f32,
) -> Element<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Shrink)
        .style(move |_theme| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, alpha))),
            border: iced::Border {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.08 * alpha),
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25 * alpha),
                offset: iced::Vector::new(0.0, 16.0),
                blur_radius: 48.0,
            },
            ..container::Style::default()
        })
        .into()
}
