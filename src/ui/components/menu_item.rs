use iced::widget::{button, row, text, Space};
use iced::{Alignment, Color, Element};
use crate::ui::widget::Message;

pub fn artifact_create_button<'a>(alpha: f32) -> Element<'a, Message> {
    button(
        row![
            text("+").size(14),
            Space::new().width(8),
            text("Artifact").size(13),
        ]
        .align_y(Alignment::Center)
    )
    .on_press(Message::CreateArtifact)
    .padding([8, 16])
    .style(move |_theme, status| {
        let is_hovered = status == button::Status::Hovered;
        button::Style {
            background: if is_hovered {
                Some(Color::from_rgba(0.0, 0.0, 0.0, 0.05 * alpha).into())
            } else {
                Some(Color::from_rgba(0.0, 0.0, 0.0, 0.02 * alpha).into())
            },
            border: iced::Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            text_color: Color::BLACK,
            shadow: iced::Shadow::default(),
            ..Default::default()
        }
    })
    .into()
}
