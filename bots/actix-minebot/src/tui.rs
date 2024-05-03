use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};
use tui_logger::TuiLoggerWidget;

use crate::app;

mod events;
use events::Event;

pub(crate) mod ui;

pub fn init(app: app::App) -> Tui {
    Tui::new(app)
}

#[derive(Default, Clone, Copy, Display, EnumIter, FromRepr, PartialEq)]
enum Tab {
    #[default]
    Logs,
    BotOps,
    Game,
    Fleets,
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

pub struct Tui {
    app: app::App,
    events: events::EventHandler,
    tab: Tab,
}

impl Tui {
    pub fn new(app: app::App) -> Self {
        let events = events::EventHandler::new();
        Tui {
            app,
            events,
            tab: Tab::default(),
        }
    }

    pub async fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.app.is_running() {
            self.update().await?;
            self.draw(terminal)?;
        }

        Ok(())
    }

    async fn update(&mut self) -> Result<()> {
        let event = self.events.next().await?;
        // log::info!("{:?}", event);

        match event {
            Event::Tick => {
                self.app.tick()?;
            }
            Event::Key(key) => self.handle_key_press(key),
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;

        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        use KeyCode::*;

        match key.code {
            Char('q') | Esc => self.app_quit(),
            Char('h') | Left => self.prev_tab(),
            Char('l') | Right => self.next_tab(),
            _ => {}
        }
    }

    fn app_quit(&mut self) {
        self.app.quit();
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

        let [header, tab_bar, content, footer] = vertical.areas(area);

        // render header
        let lines = vec![
            Line::from(format!("Welcome to Actix-Minebot! Press 'q' to quit.")),
            Line::from(format!("Game: {}", self.app.data.game_ui.0.to_string())),
            Line::from(format!("Elapsed Time: {:?}", self.app.stopwatch.elapsed())),
        ];

        Paragraph::new(Text::from(lines)).render(header, buf);

        // render tab bar
        let horizontal = Layout::horizontal([Constraint::Fill(1)]);

        let [tabs] = horizontal.areas(tab_bar);

        let tab_titles = Tab::iter().map(Tab::title);
        Tabs::new(tab_titles)
            .select(self.tab as usize)
            .render(tabs, buf);

        // render content
        match self.tab {
            Tab::Logs => {
                TuiLoggerWidget::default().render(content, buf);
            }
            Tab::BotOps => {
                let table = self.app.data.bot_ops.table();
                Paragraph::new(Text::raw(format!("{table}"))).render(content, buf);
            }
            Tab::Game => {
                let table = self.app.data.game_ui.table();
                Paragraph::new(Text::raw(format!("{table}"))).render(content, buf);
            }
            Tab::Fleets => {
                let table = self.app.data.fleets_ui.table();
                Paragraph::new(Text::raw(format!("{table}"))).render(content, buf);
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
