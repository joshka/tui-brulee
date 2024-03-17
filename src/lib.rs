use taffy::{style_helpers::FromLength, AvailableSpace};

pub fn to_rect(layout: &taffy::Layout) -> ratatui::layout::Rect {
    ratatui::layout::Rect {
        x: layout.location.x as u16,
        y: layout.location.y as u16,
        width: layout.size.width as u16,
        height: layout.size.height as u16,
    }
}

pub fn to_available_space(rect: ratatui::layout::Rect) -> taffy::Size<AvailableSpace> {
    taffy::Size {
        width: AvailableSpace::from_length(rect.width),
        height: AvailableSpace::from_length(rect.height),
    }
}
