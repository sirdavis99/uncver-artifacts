use iced::widget::{container, row, text, stack, Space};
use iced::{Background, Color, Element, Length, Font, Alignment};
use super::modal_button::close_icon_btn;
use crate::ui::widget::Message;

/// Full modal overlay including scrim and centered container.
/// This decouples the modal rendering logic from the main application view.
pub fn modal_overlay<'a>(
    main_view: impl Into<Element<'a, Message>>,
    modal: impl Into<Element<'a, Message>>,
    alpha: f32,
) -> Element<'a, Message> {
    // Semi-transparent scrim for depth
    let scrim = container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(
                Color::from_rgba(0.0, 0.0, 0.0, 0.18 * alpha)
            )),
            ..Default::default()
        });

    let overlay = container(
        container(modal.into())
            .width(380)
            .max_width(380)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    stack![main_view.into(), scrim, overlay].into()
}

/// The inner frame container for a modal.
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

/// A standardized header for all modals.
pub fn modal_header<'a>(title: &'a str, alpha: f32) -> Element<'a, Message> {
    row![
        text(title)
            .size(18)
            .font(Font {
                family: iced::font::Family::SansSerif,
                weight: iced::font::Weight::Bold,
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
