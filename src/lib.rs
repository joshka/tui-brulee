use ratatui::layout::Rect;
use taffy::{style_helpers::FromLength, AvailableSpace};

pub trait ToRect {
    fn to_rect(&self) -> Rect;
}

impl ToRect for taffy::Layout {
    fn to_rect(&self) -> Rect {
        to_rect(self)
    }
}

pub fn to_rect(layout: &taffy::Layout) -> Rect {
    Rect {
        x: layout.location.x as u16,
        y: layout.location.y as u16,
        width: layout.size.width as u16,
        height: layout.size.height as u16,
    }
}

pub trait ToAvailableSpace {
    fn to_available_space(&self) -> taffy::Size<AvailableSpace>;
}

impl ToAvailableSpace for Rect {
    fn to_available_space(&self) -> taffy::Size<AvailableSpace> {
        to_available_space(*self)
    }
}

pub fn to_available_space(rect: Rect) -> taffy::Size<AvailableSpace> {
    taffy::Size {
        width: AvailableSpace::from_length(rect.width),
        height: AvailableSpace::from_length(rect.height),
    }
}
