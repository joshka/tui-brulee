mod common;

use std::time::Duration;

use color_eyre::Result;
use common::{
    image::{image_measure_function, ImageContext},
    text::{text_measure_function, TextContext, LOREM_IPSUM},
};
use ratatui::{
    crossterm::event::{self, Event},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use taffy::{
    prelude::{auto, Display, FlexDirection, NodeId, Size, TaffyTree},
    style_helpers::max_content,
};
use tui_brulee::{to_available_space, to_rect};

enum NodeContext {
    Text(TextContext),
    Image(ImageContext),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut taffy: TaffyTree<NodeContext> = TaffyTree::new();
    taffy.enable_rounding();

    let text_node = taffy.new_leaf_with_context(
        taffy::Style {
            ..Default::default()
        },
        NodeContext::Text(TextContext {
            text_content: LOREM_IPSUM.into(),
        }),
    )?;

    let image_node = taffy.new_leaf_with_context(
        taffy::Style::default(),
        NodeContext::Image(ImageContext {
            width: 40.0,
            height: 30.0,
        }),
    )?;

    let root = taffy.new_with_children(
        taffy::Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            size: Size {
                width: auto(),
                height: auto(),
            },
            ..Default::default()
        },
        &[text_node, image_node],
    )?;

    // Compute layout and print result
    taffy.compute_layout_with_measure(
        root,
        max_content(),
        // Note: this closure is a FnMut closure and can be used to borrow external context for the
        // duration of layout For example, you may wish to borrow a global font registry and pass it
        // into your text measuring function
        |known_dimensions, available_space, _node_id, node_context, _style| {
            measure_function(known_dimensions, available_space, node_context)
        },
    )?;
    taffy.print_tree(root);

    let mut terminal = common::tui::init()?;
    loop {
        terminal.draw(|frame| {
            render(frame, &mut taffy, root, text_node, image_node).expect("render failed");
        })?;
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }
    Ok(())
}

fn render(
    frame: &mut Frame,
    taffy: &mut TaffyTree<NodeContext>,
    root: NodeId,
    text_node: NodeId,
    image_node: NodeId,
) -> Result<()> {
    taffy.compute_layout_with_measure(
        root,
        to_available_space(frame.size()),
        |known_dimensions, available_space, _node_id, node_context, _style| {
            measure_function(known_dimensions, available_space, node_context)
        },
    )?;

    let text_layout = taffy.layout(text_node)?;
    let image_layout = taffy.layout(image_node)?;

    let text_rect = to_rect(text_layout);
    let image_rect = to_rect(image_layout);

    // frame.render_widget(Block::bordered().title("Root"), root_area);
    frame.render_widget(
        Paragraph::new(LOREM_IPSUM).wrap(Wrap { trim: false }),
        text_rect,
    );
    frame.render_widget(Block::bordered().title("Image"), image_rect);

    Ok(())
}

fn measure_function(
    known_dimensions: taffy::geometry::Size<Option<f32>>,
    available_space: taffy::geometry::Size<taffy::style::AvailableSpace>,
    node_context: Option<&mut NodeContext>,
) -> Size<f32> {
    if let Size {
        width: Some(width),
        height: Some(height),
    } = known_dimensions
    {
        return Size { width, height };
    }

    match node_context {
        None => Size::ZERO,
        Some(NodeContext::Text(text_context)) => {
            text_measure_function(known_dimensions, available_space, &*text_context)
        }
        Some(NodeContext::Image(image_context)) => {
            image_measure_function(known_dimensions, image_context)
        }
    }
}
