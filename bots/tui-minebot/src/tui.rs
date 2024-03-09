use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use std::time::Duration;

use crate::{app, term, ui};

pub fn run(app: app::App, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
    Tui::new(app).run(terminal)
}

#[derive(Default, Clone, Copy, Display, EnumIter, FromRepr, PartialEq)]
enum Tab {
    #[default]
    Fleets,
    Logs,
}
struct Tui {
    app: app::App,
    tab: Tab,
}

impl Tab {
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn prev(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_sub(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn title(self) -> String {
        format!(" {self} ")
    }
}

impl Tui {
    fn new(app: app::App) -> Self {
        Self {
            app,
            tab: Tab::default(),
        }
    }

    fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        while self.app.is_running() {
            self.update()?;
            self.draw(terminal)?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn update(&mut self) -> anyhow::Result<()> {
        self.app.tick()
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;

        Ok(())
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 100.0);

        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => Ok(()),
        }
    }

    fn handle_key_press(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        use KeyCode::*;

        match key.code {
            Char('q') | Esc => self.app.quit(),
            Char('h') | Left => self.prev_tab(),
            Char('l') | Right => self.next_tab(),
            _ => {}
        };

        Ok(())
    }

    fn prev_tab(&mut self) {
        self.tab = self.tab.prev();
    }

    fn next_tab(&mut self) {
        self.tab = self.tab.next();
    }
}

impl Widget for &mut Tui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);

        let [header, tab_bar, tab_area, footer] = vertical.areas(area);

        // render header
        Paragraph::new(Text::from(vec![
            Line::from(format!("Welcome to Minebot! Press 'q' to quit.")),
            Line::from(format!("Game: {}", self.app.sage_labs.ctx.game_id)),
            Line::from(format!("Elapsed Time: {:?}", self.app.stopwatch.elapsed())),
        ]))
        .render(header, buf);

        // render tab bar
        let horizontal = Layout::horizontal([Constraint::Fill(1)]);

        let [tabs] = horizontal.areas(tab_bar);

        let tab_titles = Tab::iter().map(Tab::title);
        Tabs::new(tab_titles)
            .select(self.tab as usize)
            .render(tabs, buf);

        // render tab body
        match self.tab {
            Tab::Fleets => {
                let bots = self.app.bots();
                let tab = ui::tabs::FleetsTab::new(bots);
                tab.render(tab_area, buf);
            }
            Tab::Logs => {
                let tab = ui::tabs::LogsTab::default();
                tab.render(tab_area, buf);
            }
        }

        // render footer
        let keys = [("H/←", "Left"), ("L/→", "Right"), ("Q/Esc", "Quit")];

        let spans = keys
            .iter()
            .map(|(key, desc)| Span::default().content(format!(" {desc} ({key}) ")))
            .collect_vec();

        Line::from(spans).centered().render(footer, buf);
    }
}
