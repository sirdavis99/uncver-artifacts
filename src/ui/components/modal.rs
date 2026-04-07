use iced::widget::{container, row, text};
use iced::{Background, Color, Element, Length, Font, Alignment};
use super::modal_button::close_icon_btn;
use crate::ui::widget::Message;

pub fn modal_frame<'a, Message: 'a>(content: impl Into<Element<'a, Message>>, alpha: f32) -> Element<'a, Message> {
    container(content.into())
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, alpha))),
            border: iced::Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.20 * alpha),
                offset: iced::Vector::new(0.0, 10.0),
                blur_radius: 20.0,
            },
            ..Default::default()
        })
        .into()
}

pub fn modal_header<'a>(title: &'a str, alpha: f32) -> Element<'a, Message> {
    row![
        text(title)
            .size(20)
            .font(Font {
                family: iced::font::Family::SansSerif,
                weight: iced::font::Weight::Semibold,
                ..iced::Font::DEFAULT
            })
            .color(Color::from_rgba(0.08, 0.08, 0.08, alpha))
            .width(Length::Fill),
        close_icon_btn(Message::CloseCreateModal, alpha),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}
