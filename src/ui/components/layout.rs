use iced::widget::Space;
use iced::Length;

/// Standard vertical spacing component.
pub fn vspace(height: impl Into<Length>) -> Space {
    Space::new().height(height)
}

/// Standard horizontal spacing component.
pub fn hspace(width: impl Into<Length>) -> Space {
    Space::new().width(width)
}
