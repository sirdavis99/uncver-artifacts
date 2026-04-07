use iced::widget::{button, row, text, Column};
use iced::{Alignment, Background, Color, Font};
use crate::ui::widget::Message;
use super::inputfield::field_label;

pub fn folder_picker_row<'a>(
    label: &'a str,
    path: Option<&'a std::path::Path>,
    alpha: f32,
) -> iced::widget::Column<'a, Message> {
    let folder_str = path
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "No folder selected".to_string());

    let picker = row![
        button(
            text("Change Destination").size(11).font(Font::default())
                .color(Color::from_rgba(0.25, 0.25, 0.25, alpha))
        )
        .on_press(Message::SelectCreateFolder)
        .padding([5, 10])
        .style(move |_theme, status| button::Style {
            background: Some(Background::Color(match status {
                button::Status::Hovered => Color::from_rgba(0.86, 0.86, 0.86, alpha),
                _ => Color::from_rgba(0.92, 0.92, 0.92, alpha),
            })),
            border: iced::Border { 
                radius: 6.0.into(), 
                width: 1.0, 
                color: Color::from_rgba(0.78, 0.78, 0.78, alpha) 
            },
            text_color: Color::from_rgba(0.15, 0.15, 0.15, alpha),
            ..Default::default()
        }),
        text(folder_str).size(11).font(Font::default())
            .color(Color::from_rgba(0.5, 0.5, 0.5, alpha)),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    Column::new()
        .push(field_label(label, alpha))
        .push(picker)
        .spacing(8)
        .into()
}
