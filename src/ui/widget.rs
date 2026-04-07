use crate::ui::state::{Corner, Position, State, Trigger, WidgetMode};

use iced::widget::{container, row, text, text_input, Space};
use iced::{window, Alignment, Color, Element, Font, Length, Pixels, Task, Theme};

pub const COLLAPSED_SIZE: f32 = 48.0;
pub const EXPANDED_WIDTH: f32 = 380.0;
pub const EXPANDED_HEIGHT: f32 = 48.0;

/// Extra transparent padding on all sides so drop shadows are never clipped by the OS window.
pub const WINDOW_PAD: f32 = 40.0;
pub const WINDOW_W: f32 = EXPANDED_WIDTH + WINDOW_PAD * 2.0; // 460
pub const WINDOW_H: f32 = 300.0; // Increased to fit search results card

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
    OpenArtifact(String),
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
            Message::Hover(hovered) => {
                self.state.is_hovered = hovered;
            }
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
            Message::OpenArtifact(title) => {
                tracing::info!("Opening artifact: {}", title);
                return Task::none();
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
        let icon_btn = crate::ui::components::search_icon_button();

        let pill: Element<'_, Message> = if p < 0.1 {
            // CIRCLE MODE: icon perfectly centered inside a 48px circle
            container(icon_btn)
                .width(Length::Fixed(COLLAPSED_SIZE))
                .height(Length::Fixed(COLLAPSED_SIZE))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(move |_theme: &Theme| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, bg_alpha).into()),
                    border: iced::Border {
                        radius: iced::border::Radius::from(radius),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: pill_shadow,
                    ..Default::default()
                })
                .into()
        } else {
            // EXPANDED MODE: icon + input, fully left-aligned inside dynamic pill_w
            let input_alpha = (p * 2.0 - 0.5).clamp(0.0, 1.0);

            // Dynamically calculate input field width to prevent row overflow
            let shows_clear = !self.state.input_text.is_empty();
            let mut input_w = pill_w - 4.0 - 8.0; // Subtract total padding
            input_w -= 28.0 + 4.0; // Subtract search icon and initial gap
            if shows_clear {
                input_w -= 6.0 + 26.0; // Subtract clear button gap and button width
            }
            input_w = input_w.max(1.0);

            let input_field = text_input("Search artifacts...", &self.state.input_text)
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
                .size(18)
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

            let mut expanded_row = iced::widget::Row::new()
                .push(Space::new().width(12.0))
                .push(icon_btn)
                .push(Space::new().width(10.0))
                .push(input_field)
                .spacing(0)
                .align_y(Alignment::Center);

            if shows_clear {
                expanded_row = expanded_row
                    .push(crate::ui::components::clear_button(input_alpha))
                    .push(Space::new().width(12.0));
            } else {
                expanded_row = expanded_row.push(Space::new().width(12.0));
            }

            container(expanded_row)
                .width(Length::Fixed(pill_w))
                .height(Length::Fixed(pill_h))
                .align_x(Alignment::Start)
                .align_y(Alignment::Center)
                .padding(iced::Padding { left: 4.0, top: 0.0, right: 8.0, bottom: 0.0 })
                .style(move |_theme: &Theme| container::Style {
                    background: Some(Color::from_rgba(1.0, 1.0, 1.0, bg_alpha).into()),
                    border: iced::Border {
                        radius: iced::border::Radius::from(radius),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    shadow: pill_shadow,
                    ..Default::default()
                })
                .into()
        };

        // ── Main UI Assembly ──────────────────────────────────────
        let content_col = iced::widget::Column::new()
            .push(self.view_results_card(p))
            .push(Space::new().height(12.0))
            .push(pill)
            .align_x(Alignment::Center);

        iced::widget::mouse_area(
            container(content_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .padding(WINDOW_PAD)
                .style(|_theme: &Theme| container::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
        )
        .on_enter(Message::Hover(true))
        .on_exit(Message::Hover(false))
        .into()
    }

    fn view_results_card(&self, p: f32) -> Element<'_, Message> {
        let alpha = (p * 4.0 - 3.0).clamp(0.0, 1.0); // Fades in only at the very end of expansion
        
        if alpha <= 0.0 {
            return Space::new().height(0.0).into();
        }

        let header = iced::widget::Row::new()
            .push(text("RECOMMENDED ARTIFACTS")
                .size(10)
                .color(Color::from_rgba(0.5, 0.5, 0.5, alpha))
            )
            .padding(iced::Padding { left: 16.0, top: 12.0, right: 16.0, bottom: 4.0 });

        let results_list = iced::widget::Column::new()
            .push(header)
            .push(crate::ui::components::result_item("Podman Desktop", "Local container management", alpha))
            .push(crate::ui::components::result_item("Iced Documentation", "Cross-platform GUI library", alpha))
            .push(Space::new().height(8.0))
            .spacing(4.0);

        container(results_list)
            .width(Length::Fixed(EXPANDED_WIDTH))
            .style(move |_theme| container::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, alpha).into()),
                border: iced::Border {
                    radius: 24.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.15 * alpha),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                ..Default::default()
            })
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
