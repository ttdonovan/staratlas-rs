use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute, terminal,
};
use ratatui::prelude::*;

use std::io;
use std::time::Duration;

pub fn init() -> Result<Terminal<impl Backend>> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

pub fn restore() -> Result<()> {
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn next_event(timeout: Duration) -> Result<Option<Event>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    let event = event::read()?;
    Ok(Some(event))
}
