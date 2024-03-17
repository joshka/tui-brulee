//! Ratatui equivalent of the `flexbox_gap` example in the Taffy repository.
//!
//! Creates three 20px x 20px children, evenly spaced 10px apart from each other with a 1px border
//! around the root. Thus the container is 82px x 22px.

use std::time::Duration;

use color_eyre::Result;
use common::{errors, tui};
use crossterm::event::{self, Event};
use ratatui::{prelude::*, widgets::Block};
use taffy::prelude::*;
use tui_brulee::{to_available_space, to_rect};

mod common;

fn main() -> Result<()> {
    errors::install_hooks()?;

    let mut taffy: TaffyTree<()> = TaffyTree::new();
    taffy.enable_rounding();

    let child_style = taffy::Style {
        size: Size::from_lengths(20.0, 20.0),
        ..Default::default()
    };
    let children = &[
        taffy.new_leaf(child_style.clone())?,
        taffy.new_leaf(child_style.clone())?,
        taffy.new_leaf(child_style.clone())?,
    ];
    let root = taffy.new_with_children(
        taffy::Style {
            border: length(1.0),
            gap: taffy::Size {
                width: length(10.0),
                height: zero(),
            },
            ..Default::default()
        },
        children,
    )?;

    let (mut terminal, _guard) = tui::init()?;
    loop {
        terminal.draw(|frame| {
            render(frame, &mut taffy, root, children).expect("render failed");
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
    taffy: &mut TaffyTree,
    root: NodeId,
    children: &[NodeId],
) -> Result<()> {
    taffy.compute_layout(root, to_available_space(frame.size()))?;

    let root_layout = taffy.layout(root)?;
    let root_rect = to_rect(root_layout);
    let root_block = Block::bordered().title("Root").on_red();
    frame.render_widget(root_block, root_rect);

    for (i, child) in children.iter().enumerate() {
        let child_layout = taffy.layout(*child)?;
        let child_rect = to_rect(child_layout);
        let bg_color = Color::Indexed(i as u8 + 2);
        let child_block = Block::default().title(format!("Child {}", i)).bg(bg_color);
        frame.render_widget(child_block, child_rect);
    }
    Ok(())
}
