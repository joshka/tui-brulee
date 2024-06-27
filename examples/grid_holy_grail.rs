// This creates a so-called "holy grail" layout using the CSS Grid layout algorithm
// See: https://en.wikipedia.org/wiki/Holy_grail_(web_design)

use std::time::Duration;

use color_eyre::Result;
use common::tui;
use ratatui::{
    crossterm::event::{self, Event},
    widgets::Block,
    Frame,
};
use taffy::prelude::{NodeId, TaffyTree};
use tui_brulee::{to_available_space, to_rect};

mod common;

fn default<T: Default>() -> T {
    Default::default()
}

#[derive(Debug, Clone, Copy)]
struct NodeIds {
    root: NodeId,
    header: NodeId,
    left_sidebar: NodeId,
    content_area: NodeId,
    right_sidebar: NodeId,
    footer: NodeId,
}
fn main() -> Result<()> {
    use taffy::prelude::*;

    let mut taffy: TaffyTree<()> = TaffyTree::new();

    // Setup the grid
    let root_style = Style {
        display: Display::Grid,
        size: Size::from_percent(1.0, 1.0),
        border: length(1.0),
        grid_template_columns: vec![length(20.0), fr(1.0), length(20.0)],
        grid_template_rows: vec![length(3.0), fr(1.0), length(3.0)],
        ..default()
    };

    // Define the child nodes

    let header = taffy.new_leaf(Style {
        grid_row: line(1),
        grid_column: span(3),
        ..default()
    })?;
    let left_sidebar = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(1),
        ..default()
    })?;
    let content_area = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(2),
        ..default()
    })?;
    let right_sidebar = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(3),
        ..default()
    })?;
    let footer = taffy.new_leaf(Style {
        grid_row: line(3),
        grid_column: span(3),
        ..default()
    })?;

    // Create the container with the children
    let root = taffy.new_with_children(
        root_style,
        &[header, left_sidebar, content_area, right_sidebar, footer],
    )?;

    let node_ids = NodeIds {
        root,
        header,
        left_sidebar,
        content_area,
        right_sidebar,
        footer,
    };

    // Compute layout and print result
    taffy.compute_layout(
        root,
        Size {
            width: length(800.0),
            height: length(600.0),
        },
    )?;

    let (mut terminal, _guard) = tui::init()?;
    loop {
        terminal.draw(|frame| {
            render(frame, &mut taffy, node_ids).expect("render failed");
        })?;
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, taffy: &mut TaffyTree, node_ids: NodeIds) -> Result<()> {
    taffy.compute_layout(node_ids.root, to_available_space(frame.size()))?;

    let root_layout = taffy.layout(node_ids.root)?;
    let root_rect = to_rect(root_layout);
    let root_block = Block::bordered().title("Root");
    frame.render_widget(root_block, root_rect);

    for (node_id, title) in [
        (node_ids.header, "Header"),
        (node_ids.left_sidebar, "Left Sidebar"),
        (node_ids.content_area, "Content Area"),
        (node_ids.right_sidebar, "Right Sidebar"),
        (node_ids.footer, "Footer"),
    ] {
        let child_layout = taffy.layout(node_id)?;
        let child_rect = to_rect(child_layout);
        let child_block = Block::bordered().title(title);
        frame.render_widget(child_block, child_rect);
    }
    Ok(())
}
