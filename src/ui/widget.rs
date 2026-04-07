use crate::ui::state::{Corner, Position, State, Trigger, WidgetMode};

use iced::widget::{button, container, row, svg, text, text_input, Space};
use iced::{window, Alignment, Color, Element, Font, Length, Pixels, Task, Theme};

pub const COLLAPSED_SIZE: f32 = 48.0;
pub const EXPANDED_WIDTH: f32 = 380.0;
pub const EXPANDED_HEIGHT: f32 = 48.0;

/// Extra transparent padding on all sides so drop shadows are never clipped by the OS window.
pub const WINDOW_PAD: f32 = 40.0;
pub const WINDOW_W: f32 = EXPANDED_WIDTH + WINDOW_PAD * 2.0; // 460
pub const WINDOW_H: f32 = EXPANDED_HEIGHT + WINDOW_PAD * 2.0; // 128

const SEARCH_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.35-4.35"/></svg>"#;

#[derive(Debug, Clone)]
pub enum Message {
    Hover(bool),
    Click,
    IconClick,
    KeyPress(char),
    Backspace,
    Clear,
    Escape,
    ClickOutside,
    SnapToCorner,
    UpdateLayout { width: f32, height: f32 },
    WindowEvent(window::Id, window::Event),
    Tick,
}

pub struct SearchWidget {
    pub state: State,
    pub layout_width: f32,
    pub layout_height: f32,
    pub window_id: Option<window::Id>,
}

impl SearchWidget {
    pub fn new() -> Self {
        Self {
            state: State::default(),
            layout_width: EXPANDED_WIDTH,
            layout_height: EXPANDED_HEIGHT,
            window_id: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Hover(_hovered) => {}
            Message::Click => {
                // Clicking the dormant input area expands the pill
                if self.state.animation_progress.target == 0.0 {
                    self.state.animation_progress.target = 1.0;
                    self.state.is_animating = true;
                }
            }
            Message::IconClick => {
                // The search icon is the primary toggle
                if self.state.animation_progress.target == 0.0 {
                    self.state.animation_progress.target = 1.0;
                } else {
                    self.state.clear_input();
                    self.state.animation_progress.target = 0.0;
                }
                self.state.is_animating = true;
            }
            Message::KeyPress(c) => {
                if c == '\u{7f}' || c == '\u{8}' {
                    self.state.backspace();
                } else if c == '\u{1b}' {
                    self.state.transition(Trigger::Escape);
                    self.state.animation_progress.target = 0.0;
                    self.state.is_animating = true;
                } else if c == '\r' || c == '\n' {
                    tracing::info!("Search submitted: {}", self.state.input_text);
                } else if c.is_ascii_graphic() || c == ' ' {
                    self.state.update_input(c);
                    // Ensure we stay expanded if typing
                    self.state.animation_progress.target = 1.0;
                }
            }
            Message::Backspace => {
                self.state.backspace();
            }
            Message::Clear => {
                self.state.clear_input();
                self.state.animation_progress.target = 0.0;
                self.state.is_animating = true;
            }
            Message::Escape => {
                self.state.transition(Trigger::Escape);
                self.state.animation_progress.target = 0.0;
                self.state.is_animating = true;
            }
            Message::ClickOutside => {
                // Collapse on click outside if empty
                if self.state.input_text.is_empty() {
                    self.state.animation_progress.target = 0.0;
                    self.state.is_animating = true;
                }
            }
            Message::SnapToCorner => {
                self.state.transition(Trigger::SnapToCorner);
            }
            Message::UpdateLayout { width, height } => {
                self.layout_width = width;
                self.layout_height = height;
            }
            Message::WindowEvent(id, event) => {
                if self.window_id.is_none() {
                    if let window::Event::Opened { .. } = event {
                        self.window_id = Some(id);
                    }
                }
            }
            Message::Tick => {
                if self.state.is_animating {
                    let diff = self.state.animation_progress.target
                        - self.state.animation_progress.progress;
                    if diff.abs() < 0.001 {
                        self.state.animation_progress.progress =
                            self.state.animation_progress.target;
                        self.state.is_animating = false;
                        if self.state.animation_progress.target == 1.0 {
                            self.state.mode = WidgetMode::SearchMode;
                        } else {
                            self.state.mode = WidgetMode::Collapsed;
                        }
                    } else {
                        self.state.animation_progress.progress += diff * 0.2;
                    }
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let p = self.state.animation_progress.progress; // 0.0 (circle) → 1.0 (pill)
        
        // ── Dimensions ───────────────────────────────────────────
        let pill_w = COLLAPSED_SIZE + (EXPANDED_WIDTH - COLLAPSED_SIZE) * p;
        let pill_h = EXPANDED_HEIGHT;
        let radius = 24.0; 

        // ── Inactive / Hover State ───────────────────────────────
        // Only applies to the 'minimized' (p=0) style
        let is_idle = p < 0.01 && !self.state.is_hovered;
        let bg_alpha = if is_idle { 0.5 } else { 1.0 };
        let shadow_alpha = if is_idle { 0.05 } else { 0.18 };

        let pill_shadow = iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, shadow_alpha),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: if is_idle { 8.0 } else { 20.0 },
        };

        // ── Search Icon Button ───────────────────────────────────
        let svg_handle = svg::Handle::from_memory(SEARCH_SVG.as_bytes().to_vec());
        let icon_btn = button(
            container(svg(svg_handle).width(24).height(24))
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0))
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        )
        .on_press(Message::IconClick)
        .padding(0)
        .style(|_theme: &Theme, _status: button::Status| button::Style {
            background: None,
            ..Default::default()
        });

        // ── Content ──────────────────────────────────────────────
        let input_alpha = (p * 2.0 - 0.5).clamp(0.0, 1.0);
        
        let mut items: Vec<Element<Message>> = vec![
            icon_btn.into(),
        ];

        if p > 0.1 {
            let input_field = text_input(
                "Search artifacts...", 
                &self.state.input_text
            )
            .on_input(|s| {
                if s.len() < self.state.input_text.len() {
                    Message::Backspace
                } else if let Some(c) = s.chars().last() {
                    Message::KeyPress(c)
                } else {
                    Message::Backspace
                }
            })
            .padding(0)
            .size(18) // Tighter font
            .font(Font::DEFAULT)
            .line_height(Pixels(24.0))
            .width(Length::Fill) 
            .style(move |_theme, _| text_input::Style {
                background: Color::TRANSPARENT.into(),
                border: iced::Border { radius: 0.0.into(), width: 0.0, color: Color::TRANSPARENT },
                icon: Color::from_rgba(0.0, 0.0, 0.0, input_alpha),
                placeholder: Color::from_rgba(0.55, 0.55, 0.55, input_alpha),
                value: Color::from_rgba(0.2, 0.2, 0.2, input_alpha),
                selection: Color::from_rgba(0.78, 0.85, 1.0, input_alpha),
            });

            // Minimal spacing after icon
            items.push(Space::new().width(2.0).into());
            items.push(input_field.into());
            
            if !self.state.input_text.is_empty() {
                let clear_button = button(text("✕").size(14).font(Font::DEFAULT))
                    .on_press(Message::Clear)
                    .width(28)
                    .height(28)
                    .padding(0)
                    .style(move |_theme, _| button::Style {
                        background: Some(Color::from_rgba(0.86, 0.86, 0.86, input_alpha).into()),
                        border: iced::Border { radius: 14.0.into(), ..Default::default() },
                        ..Default::default()
                    });
                
                items.push(Space::new().width(4.0).into());
                items.push(clear_button.into());
            }
            
            items.push(Space::new().width(8.0).into());
        }

        let inner = container(
            row(items)
                .spacing(0)
                .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            left: 8.0, 
            top: 4.0, 
            right: 8.0, 
            bottom: 4.0,
        })
        .align_y(Alignment::Center)
        .align_x(Alignment::Start);

        // ── Main Pill ────────────────────────────────────────────
        let pill = container(inner)
            .width(Length::Fixed(pill_w))
            .height(Length::Fixed(pill_h))
            .style(move |_theme: &Theme| container::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, bg_alpha).into()),
                border: iced::Border {
                    radius: iced::border::Radius::from(radius),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: pill_shadow,
                ..Default::default()
            });

        // Ensure the outer container captures mouse events for hover
        container(pill)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            })
            .on_mouseenter(Message::Hover(true))
            .on_mouseleave(Message::Hover(false))
            .into()
    }
    
    pub fn get_position(&self, screen_width: f32, screen_height: f32) -> Position {
        if self.state.corner_snap != Corner::None {
            match self.state.corner_snap {
                Corner::TopRight => Position::top_right(
                    screen_width,
                    screen_height,
                    self.layout_width,
                    self.layout_height,
                    20.0,
                ),
                _ => Position::center(
                    screen_width,
                    screen_height,
                    self.layout_width,
                    self.layout_height,
                ),
            }
        } else {
            Position::center(
                screen_width,
                screen_height,
                self.layout_width,
                self.layout_height,
            )
        }
    }
}

impl Default for SearchWidget {
    fn default() -> Self {
        Self::new()
    }
}
