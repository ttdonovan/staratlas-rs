use anyhow::Result;
use crossterm::{execute, terminal};
use ratatui::prelude::*;

use std::io;

pub fn init() -> Result<Terminal<impl Backend>> {
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        terminal::Clear,
        terminal::EnterAlternateScreen
    )?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

pub fn restore() -> Result<()> {
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
