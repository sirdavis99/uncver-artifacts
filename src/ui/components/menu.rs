use iced::widget::{container, row, scrollable};
use iced::{Element, Length, Padding};
use crate::ui::widget::Message;
use super::menu_item;
use super::icons;

pub fn create_artifact_menu<'a>(alpha: f32) -> Element<'a, Message> {
    container(
        scrollable(
            row![
                menu_item::menu_button("Create Artifact".to_string(), icons::CREATE_SVG, Some(Message::CreateArtifact), alpha),
                menu_item::menu_button("Image".to_string(), icons::IMAGE_SVG, None, alpha),
                menu_item::menu_button("Video".to_string(), icons::VIDEO_SVG, None, alpha),
                menu_item::menu_button("Folder".to_string(), icons::FOLDER_SVG, None, alpha),
            ]
            .spacing(8)
        )
        .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::default()))
    )
    .padding(Padding { top: 2.0, right: 12.0, bottom: 6.0, left: 12.0 }) // Adjusted padding for horizontal look
    .width(Length::Fill)
    .into()
}
