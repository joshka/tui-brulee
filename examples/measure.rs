mod common;

use color_eyre::Result;
use common::{
    image::ImageContext,
    text::{TextContext, LOREM_IPSUM},
};
use ratatui::{
    crossterm::event::{self, Event},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};
use taffy::prelude::{
    auto, length, AvailableSpace, Display, FlexDirection, NodeId, Size, TaffyTree,
};
use tui_brulee::{ToAvailableSpace, ToRect};

enum NodeContext {
    Text(TextContext),
    Image(ImageContext),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new()?;
    let mut terminal = common::tui::init()?;
    let result = app.run(&mut terminal);
    common::tui::restore()?;
    result
}

struct App {
    taffy: TaffyTree<NodeContext>,
    root: NodeId,
    text_node: NodeId,
    image_node: NodeId,
}

impl App {
    fn new() -> Result<Self> {
        let mut taffy: TaffyTree<NodeContext> = TaffyTree::new();

        let text_node = taffy.new_leaf_with_context(
            taffy::Style::default(),
            NodeContext::Text(TextContext::new(LOREM_IPSUM.into())),
        )?;

        let image_node = taffy.new_leaf_with_context(
            taffy::Style::default(),
            NodeContext::Image(ImageContext::new(200.0, 150.0)),
        )?;

        let root = taffy.new_with_children(
            taffy::Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: auto(),
                ..Default::default()
            },
            &[text_node, image_node],
        )?;

        Ok(Self {
            taffy,
            root,
            text_node,
            image_node,
        })
    }

    fn run(&mut self, terminal: &mut common::tui::Terminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame).unwrap())?;
            if let Event::Key(_) = event::read()? {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        self.taffy.compute_layout_with_measure(
            self.root,
            frame.size().to_available_space(),
            |known_dimensions, available_space, _node_id, node_context, _style| {
                measure_function(known_dimensions, available_space, node_context)
            },
        )?;

        let text_area = self.taffy.layout(self.text_node)?.to_rect();
        let image_area = self.taffy.layout(self.image_node)?.to_rect();

        let paragraph = Paragraph::new(LOREM_IPSUM).wrap(Wrap { trim: false });
        let image =
            Paragraph::new(format!("{:?}", image_area)).block(Block::bordered().title("Image"));

        frame.render_widget(paragraph, text_area);
        frame.render_widget(image, image_area);

        Ok(())
    }
}

fn measure_function(
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
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
        Some(NodeContext::Text(context)) => context.measure(known_dimensions, available_space),
        Some(NodeContext::Image(context)) => context.measure(known_dimensions),
    }
}
