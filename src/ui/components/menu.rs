use iced::widget::container;
use iced::{Element, Length};
use crate::ui::widget::Message;
use super::menu_item;

pub fn create_artifact_menu<'a>(alpha: f32) -> Element<'a, Message> {
    container(menu_item::artifact_create_button(alpha))
        .padding([4, 12])
        .width(Length::Fill)
        .into()
}
