pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint \
occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

pub struct TextContext {
    pub text_content: String,
}

pub fn text_measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    text_context: &TextContext,
) -> taffy::geometry::Size<f32> {
    use taffy::prelude::*;

    let words: Vec<&str> = text_context.text_content.split_whitespace().collect();

    if words.is_empty() {
        return Size::ZERO;
    }

    let min_line_length: usize = words.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_line_length: usize = words.iter().map(|line| line.len()).sum();
    let width = known_dimensions
        .width
        .unwrap_or_else(|| match available_space.width {
            AvailableSpace::MinContent => min_line_length as f32,
            AvailableSpace::MaxContent => max_line_length as f32,
            AvailableSpace::Definite(inline_size) => inline_size
                .min(max_line_length as f32)
                .max(min_line_length as f32),
        });
    let height = known_dimensions.height.unwrap_or_else(|| {
        let inline_line_length = width.floor() as usize;
        let mut line_count = 1;
        let mut current_line_length = 0;
        for word in &words {
            if current_line_length == 0 {
                current_line_length = word.len();
            } else if current_line_length + 1 + word.len() > inline_line_length {
                line_count += 1;
                current_line_length = word.len();
            } else {
                current_line_length += word.len() + 1;
            };
        }
        line_count as f32
    });

    Size { width, height }
}
