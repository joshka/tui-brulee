use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
};

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Terminal> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn set_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        if let Err(err) = restore() {
            eprintln!("Error restoring terminal: {:?}", err);
        }
        hook(panic_info);
    }));
}
