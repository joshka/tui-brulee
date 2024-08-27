//! This example is the Ratatui equivalent of the `basic` example in the Taffy repository.
//!
//! The layout is a simple tree with two nodes, the root and a child. The root node is a flex
//! container the full size of the terminal with a 1 cell border, and the child node is a flex item
//! that takes up half of the root node's width and all of its height and is centered within the
//! root node.
//!

use std::io;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event},
    layout::Rect,
    prelude::{Frame, Stylize},
    widgets::{Block, Paragraph},
    DefaultTerminal,
};
use taffy::{
    prelude::{length, percent, AlignItems, JustifyContent, NodeId, TaffyTree},
    AvailableSpace, TaffyResult,
};
use tui_brulee::{ToAvailableSpace, ToRect};

mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new()?;
    let result = app.run(terminal);
    ratatui::restore();
    result
}

struct App {
    taffy: TaffyTree<()>,
    root: NodeId,
    child: NodeId,
}

impl App {
    fn new() -> Result<Self> {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        let root = taffy.new_leaf(taffy::Style {
            size: percent(1.0),
            border: length(1.0),
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            ..Default::default()
        })?;

        let child = taffy.new_leaf(taffy::Style {
            size: percent(0.5),
            ..Default::default()
        })?;
        taffy.add_child(root, child)?;

        Ok(Self { taffy, root, child })
    }

    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.try_draw(|frame| self.draw(frame))?;
            if let Event::Key(_) = event::read()? {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> io::Result<()> {
        let (root_area, child_rect) = self
            .compute_layout(frame.area().to_available_space())
            .map_err(common::to_io_error)?;

        let root_block = Block::bordered().title("Root").on_blue();
        let child_block = Block::bordered().title("Child").on_red();
        let child_paragraph = Paragraph::new("Press any key to exit").block(child_block);

        frame.render_widget(root_block, root_area);
        frame.render_widget(child_paragraph, child_rect);

        Ok(())
    }

    fn compute_layout(
        &mut self,
        available_space: taffy::Size<AvailableSpace>,
    ) -> TaffyResult<(Rect, Rect)> {
        self.taffy.compute_layout(self.root, available_space)?;
        let root_area = self.taffy.layout(self.root)?.to_rect();
        let child_rect = self.taffy.layout(self.child)?.to_rect();
        Ok((root_area, child_rect))
    }
}
