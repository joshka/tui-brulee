//! This example is the Ratatui equivalent of the `basic` example in the Taffy repository.
//!
//! The layout is a simple tree with two nodes, the root and a child. The root node is a flex
//! container the full size of the terminal with a 1 cell border, and the child node is a flex item
//! that takes up half of the root node's width and all of its height and is centered within the
//! root node.
//!

use std::time::Duration;

use color_eyre::Result;
use common::{errors, tui};
use crossterm::event::{self, Event};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};
use taffy::prelude::*;
use tui_brulee::{to_available_space, to_rect};

mod common;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    taffy.enable_rounding();

    let child = taffy.new_leaf(taffy::Style {
        size: percent(0.5),
        ..Default::default()
    })?;

    let root = taffy.new_with_children(
        taffy::Style {
            size: percent(1.0),
            border: length(1.0),
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            ..Default::default()
        },
        &[child],
    )?;

    let (mut terminal, _guard) = tui::init()?;
    loop {
        terminal.draw(|frame| draw(frame, &mut taffy, root, child).unwrap())?;
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, taffy: &mut TaffyTree<()>, root: NodeId, child: NodeId) -> Result<()> {
    taffy.compute_layout(root, to_available_space(frame.size()))?;

    let root_area = to_rect(taffy.layout(root)?);
    let child_rect = to_rect(taffy.layout(child)?);

    let root_block = Block::bordered().title("Root").on_blue();
    let child_block = Block::bordered().title("Child").on_red();
    let child_paragraph = Paragraph::new("Press any key to exit").block(child_block);

    frame.render_widget(root_block, root_area);
    frame.render_widget(child_paragraph, child_rect);

    Ok(())
}
