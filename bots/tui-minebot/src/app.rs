use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use std::time::{Duration, Instant};

use crate::{bots, sage, term, time};

pub struct App {
    mode: Mode,
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
            _ => {}
        };

        Ok(())
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
        let vertical = Layout::vertical([Constraint::Length(4), Constraint::Min(0)]);

        let [header, body] = vertical.areas(area);

        // render header
        Paragraph::new(Text::from(vec![
            Line::from(format!("Welcome to Minebot! Press 'q' to quit.")),
            Line::from(format!("Game: {}", self.sage_ctx.game_id)),
            Line::from(format!("Elapsed Time: {:?}", self.stopwatch.elapsed())),
        ]))
        .render(header, buf);

        // render body
        let mut table = comfy_table::Table::new();
        table.set_header(vec![
            "Fleet ID",
            "Name/Callsign",
            "Resource Name",
            "Mining Rate",
            "Mining Amount",
            "Mining Duration",
            "Mining Timer",
            "Fuel Status",
            "Ammo Status",
            "Cargo Status",
            "Autoplay",
        ]);

        let mut tx_table = comfy_table::Table::new();
        tx_table.set_header(vec!["Fleet ID", "Last Txs", "Counter", "Errors"]);

        for bot in &self.bots {
            table.add_row(vec![
                format!("{}", bot.masked_fleet_id()),
                format!("{}", bot.fleet_name()),
                format!("{}", bot.mine_item_name()),
                format!("{}", bot.mine_rate()),
                format!("{}", bot.mine_amount()),
                format!("{:.2}s", bot.mine_duration().as_secs_f32()),
                format!(
                    "{:.2}s ({:.3})",
                    bot.mining_timer.remaining_secs(),
                    bot.mining_timer.fraction()
                ),
                format!("{}/{}", bot.fuel_tank.1, bot.fuel_tank.2),
                format!("{}/{}", bot.ammo_bank.1, bot.ammo_bank.2),
                format!("{}/{}", bot.cargo_hold.1, bot.cargo_hold.2),
                format!("{:#?}", bot.autoplay),
            ]);

            tx_table.add_row(vec![
                format!("{}", bot.masked_fleet_id()),
                format!("{}", bot.last_txs()),
                format!("{}", bot.txs_counter),
                format!("{}", bot.txs_errors),
            ]);
        }

        Paragraph::new(Text::raw(format!("{table}\n{tx_table}"))).render(body, buf);
    }
}
