//! Ratatui equivalent of the `flexbox_gap` example in the Taffy repository.
//!
//! Creates three 20px x 10px children, evenly spaced 10px apart from each other with a 1px border
//! around the root. Thus the container is 82px x 12px.

use std::time::Duration;

use color_eyre::Result;
use common::tui;
use ratatui::{
    crossterm::event::{self, Event},
    layout::Rect,
    style::{Color, Stylize},
    widgets::Block,
    Frame,
};
use taffy::prelude::{length, zero, NodeId, Size, TaffyTree};
use tui_brulee::{ToAvailableSpace, ToRect};

mod common;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new()?;
    let mut terminal = tui::init()?;
    let result = app.run(&mut terminal);
    tui::restore()?;
    result
}

struct App {
    taffy: TaffyTree<()>,
    root: NodeId,
    children: [NodeId; 3],
}

impl App {
    fn new() -> Result<Self> {
        let mut taffy: TaffyTree<()> = TaffyTree::new();

        let child_style = taffy::Style {
            size: Size::from_lengths(20.0, 10.0),
            ..Default::default()
        };
        let children = [
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
            &children,
        )?;

        Ok(Self {
            taffy,
            root,
            children,
        })
    }

    fn run(&mut self, terminal: &mut tui::Terminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame).unwrap())?;
            if event::poll(Duration::from_secs(1))? {
                if let Event::Key(_) = event::read()? {
                    break Ok(());
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let (root_area, child_rects) = self.compute_layout(frame.size())?;

        let root_block = Block::bordered().title("Root").on_red();
        frame.render_widget(root_block, root_area);

        for (i, area) in child_rects.iter().enumerate() {
            let bg_color = Color::Indexed(i as u8 + 2); // Green, Yellow, Blue
            let child_block = Block::default().title(format!("Child {}", i)).bg(bg_color);
            frame.render_widget(child_block, *area);
        }
        Ok(())
    }

    fn compute_layout(&mut self, area: Rect) -> Result<(Rect, Vec<Rect>)> {
        let available_space = area.to_available_space();
        self.taffy.compute_layout(self.root, available_space)?;
        let root_area = self.taffy.layout(self.root)?.to_rect();
        let child_rects = self
            .children
            .iter()
            .map(|child| self.taffy.layout(*child).map(ToRect::to_rect))
            .flatten()
            .collect();
        Ok((root_area, child_rects))
    }
}
