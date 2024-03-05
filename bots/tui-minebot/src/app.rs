use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use std::time::{Duration, Instant};

use crate::{bots, sage, tabs, term, time};

pub struct App {
    mode: Mode,
    tab: Tab,
    stopwatch: time::Stopwatch,
    dt: Duration,
    last_time: Instant,
    sage_ctx: sage::SageContext,
    bots: Vec<bots::MiningBot>,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

#[derive(Default, Clone, Copy, Display, EnumIter, FromRepr, PartialEq)]
enum Tab {
    #[default]
    Fleets,
    Logs,
}

pub fn run(
    context: sage::SageContext,
    bots: Vec<bots::MiningBot>,
    terminal: &mut Terminal<impl Backend>,
) -> Result<()> {
    App::new(context, bots).run(terminal)
}

impl App {
    pub fn new(context: sage::SageContext, bots: Vec<bots::MiningBot>) -> Self {
        App {
            mode: Mode::default(),
            tab: Tab::default(),
            stopwatch: time::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            sage_ctx: context,
            bots,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            self.update()?;
            self.draw(terminal)?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 100.0);

        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => Ok(()),
        }
    }

    fn handle_key_press(&mut self, key: KeyEvent) -> Result<()> {
        use KeyCode::*;

        match key.code {
            Char('q') | Esc => self.quit(),
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

    fn update(&mut self) -> Result<()> {
        // calculate delta time
        let now = Instant::now();
        self.dt = now.duration_since(self.last_time);
        self.last_time = now;

        // update app timers
        self.stopwatch.tick(self.dt);

        // for each bot run update
        for bot in &mut self.bots {
            bots::run_update(bot, self.dt, &self.sage_ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;

        Ok(())
    }

    fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}

impl Widget for &mut App {
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
            Line::from(format!("Game: {}", self.sage_ctx.game_id)),
            Line::from(format!("Elapsed Time: {:?}", self.stopwatch.elapsed())),
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
                let tab = tabs::FleetsTab::new(&self.bots);
                tab.render(tab_area, buf);
            }
            Tab::Logs => {
                let tab = tabs::LogsTab::default();
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
