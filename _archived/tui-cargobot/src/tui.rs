use crossterm::event::{KeyCode, KeyEvent};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use tui_logger::TuiLoggerWidget;

use crate::app;

mod events;
use events::Event;

pub async fn run(app: app::App, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
    Tui::new(app).run(terminal).await
}

struct Tui {
    app: app::App,
    events: events::EventHandler,
}

impl Tui {
    pub fn new(app: app::App) -> Self {
        let events = events::EventHandler::new();
        Tui { app, events }
    }

    async fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.app.is_running() {
            self.update().await?;
            self.draw(terminal)?;
        }

        Ok(())
    }

    async fn update(&mut self) -> anyhow::Result<()> {
        let event = self.events.next().await?;
        // log::info!("{:?}", event);

        match event {
            Event::Tick => {
                self.app.tick();
            }
            Event::Key(key) => self.handle_key_press(key),
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;

        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        use KeyCode::*;

        match key.code {
            Char('q') | Esc => self.app.quit(),
            Char('r') => self.app.refresh_fleet(),
            _ => {}
        }
    }
}

impl Widget for &mut Tui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(6),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);

        let [header, body, footer] = vertical.areas(area);

        // render header
        let mut lines = vec![
            Line::from(format!("Welcome to Cargobot! Press 'q' to quit.")),
            Line::from(format!("Game: {}", self.app.game_id)),
            Line::from(format!("Elapsed Time: {:?}", self.app.stopwatch.elapsed())),
        ];

        if let Some(bot) = &self.app.bot {
            lines.push(Line::from(format!(
                "Fleet: {:?} - Sector: {:?} -> {:?} ({})",
                bot.fleet_id, bot.from_sector, bot.to_sector, bot.num_runs
            )));

            lines.push(Line::from(format!(
                "Warp Cool Down: {:?}",
                bot.timers.warp_cool_down
            )));
        }

        Paragraph::new(Text::from(lines)).render(header, buf);

        // render body
        TuiLoggerWidget::default().render(body, buf);

        // render footer
        let keys = [("Q/Esc", "Quit")];

        let spans = keys
            .iter()
            .map(|(key, desc)| Span::default().content(format!(" {desc} ({key}) ")))
            .collect_vec();

        Line::from(spans).centered().render(footer, buf);
    }
}
