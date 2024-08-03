use taffy::geometry::Size;

pub struct ImageContext {
    pub width: f32,
    pub height: f32,
}

impl ImageContext {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn measure(&self, known_dimensions: Size<Option<f32>>) -> Size<f32> {
        match (known_dimensions.width, known_dimensions.height) {
            (Some(width), Some(height)) => Size { width, height },
            (Some(width), None) => Size {
                width,
                height: (width / self.width) * self.height,
            },
            (None, Some(height)) => Size {
                width: (height / self.height) * self.width,
                height,
            },
            (None, None) => Size {
                width: self.width,
                height: self.height,
            },
        }
    }
}
