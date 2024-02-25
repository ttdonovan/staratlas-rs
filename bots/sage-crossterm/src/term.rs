use color_eyre::Result;
use crossterm::{
    cursor,
    event::{self, Event},
    execute, queue, terminal,
};

use std::io::{self, stdout};
use std::time::Duration;

pub fn init() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;

    Ok(())
}

pub fn restore() -> Result<()> {
    execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub fn clear<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    queue!(
        w,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}

pub fn flush<W>(w: &mut W) -> Result<()>
where
    W: io::Write,
{
    w.flush()?;

    Ok(())
}

pub fn next_event(timeout: Duration) -> Result<Option<Event>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    let event = event::read()?;
    Ok(Some(event))
}
