use uncver_artifacts::ui::widget::{Message, WINDOW_H, WINDOW_W};
use uncver_artifacts::SearchWidget;

use iced::time::Duration;
use iced::window::settings::PlatformSpecific;
use iced::window::{Position, Settings as WindowSettings};
use iced::{application, window, Color, Element, Size, Task};

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter("uncver_artifacts=debug,info")
        .init();

    tracing::info!("Starting uncver-artifacts...");

    let platform_specific = PlatformSpecific {
        title_hidden: true,
        titlebar_transparent: true,
        fullsize_content_view: true,
    };

    // Window has extra padding around the widget so shadows are never clipped.
    // Visible widget area expands/contracts, but window is static.
    let window_settings = WindowSettings {
        size: Size::new(WINDOW_W, WINDOW_H),
        min_size: Some(Size::new(WINDOW_W, WINDOW_H)),
        max_size: Some(Size::new(WINDOW_W, WINDOW_H)),
        position: Position::SpecificWith(|_window_size: Size, monitor_size: Size| {
            // Center horizontally, position near bottom
            let x = (monitor_size.width - WINDOW_W) / 2.0;
            let y = monitor_size.height - WINDOW_H - 100.0; // Slightly higher up for better focus
            tracing::info!(
                "Window position: x={}, y={}, screen: {:?}",
                x,
                y,
                monitor_size
            );
            iced::Point::new(x, y)
        }),
        resizable: false,
        decorations: false,
        transparent: true,
        platform_specific,
        ..Default::default()
    };

    iced::application(SearchWidget::new, update, view)
        .window(window_settings)
        .style(|_widget| application::Style {
            background: Color::TRANSPARENT.into(),
            text_color: Color::BLACK,
        })
        .subscription(subscription)
        .run()
}

fn subscription(_widget: &SearchWidget) -> iced::Subscription<Message> {
    let tick = iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick);

    let events = window::events().map(|(id, event)| Message::WindowEvent(id, event));

    iced::Subscription::batch([tick, events])
}

fn update(widget: &mut SearchWidget, message: Message) -> Task<Message> {
    widget.update(message)
}

fn view(widget: &SearchWidget) -> Element<'_, Message> {
    widget.view()
}
