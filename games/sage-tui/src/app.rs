use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use std::time::{Duration, Instant};

use crate::{sage, term, time};

pub struct App {
    mode: Mode,
    stopwatch: time::Stopwatch,
    dt: Duration,
    last_time: Instant,
    sage: sage::SageContext,
    sage_fleet: Vec<(sage::Pubkey, (sage::Fleet, sage::FleetState))>,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

pub fn run(context: sage::SageContext, terminal: &mut Terminal<impl Backend>) -> Result<()> {
    App::new(context).run(terminal)
}

impl App {
    pub fn new(context: sage::SageContext) -> Self {
        App {
            mode: Mode::default(),
            stopwatch: time::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            sage: context,
            sage_fleet: vec![],
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        self.refresh();

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
            Char('r') => self.refresh(),
            _ => {}
        };

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        // calculate delta time
        let now = Instant::now();
        self.dt = now.duration_since(self.last_time);
        self.last_time = now;

        // update timers
        self.stopwatch.tick(self.dt);

        Ok(())
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;

        Ok(())
    }

    fn refresh(&mut self) {
        self.sage_fleet = self.sage.get_fleet_with_state();
    }

    fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]);

        let [head, body] = vertical.areas(area);

        Paragraph::new(Text::from(vec![
            Line::from("Welcome to Sage! Press 'r' to refresh or 'q' to quit."),
            Line::from(format!(
                "Elapsed time: {:.2}",
                self.stopwatch.elapsed_secs_f64()
            )),
            Line::from(format!("Game: {}", self.sage.game_id)),
            Line::from(format!("Profile: {}", self.sage.profile_id)),
            Line::from(format!("Fleets: {}", self.sage_fleet.len())),
        ]))
        .render(head, buf);

        let mut table = comfy_table::Table::new();
        table.set_header(vec!["Fleet ID", "Fleet Name", "Status"]);

        for (pubkey, (fleet, state)) in &self.sage_fleet {
            table.add_row(vec![
                format!("{}", pubkey),
                format!("{}", fleet.fleet_label()),
                format!("{:#?}", state),
            ]);
        }

        Paragraph::new(Text::raw(format!("{table}"))).render(body, buf);
    }
}
