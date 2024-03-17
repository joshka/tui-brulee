use std::io::{self, stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

pub fn init() -> io::Result<(Terminal<impl Backend>, TuiGuard)> {
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok((terminal, TuiGuard))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
pub struct TuiGuard;

impl Drop for TuiGuard {
    fn drop(&mut self) {
        restore().unwrap();
    }
}
