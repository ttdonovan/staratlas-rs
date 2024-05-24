use anchor_client::anchor_lang::prelude::Pubkey;
use color_eyre::Result;
use staratlas_sage_based_sdk::{Fleet, Game};

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::{actors, db, timers, tui::ui};

pub fn init(
    db: Arc<Mutex<db::MinebotDB>>,
    game: (Pubkey, Game),
    fleets: Vec<(Pubkey, Fleet)>,
) -> App {
    App::new(
        db,
        AppData {
            game_ui: game.into(),
            fleets_ui: fleets.into(),
            bot_ops: ui::BotOpsUI::from(vec![]),
        },
    )
}

pub struct App {
    mode: Mode,
    db: Arc<Mutex<db::MinebotDB>>,
    db_timer: timers::Timer,
    pub(crate) stopwatch: timers::Stopwatch,
    dt: Duration,
    last_time: Instant,
    pub(crate) data: AppData,
}

pub struct AppData {
    pub(crate) game_ui: ui::GameUI,
    pub(crate) fleets_ui: ui::FleetsUI,
    pub(crate) bot_ops: ui::BotOpsUI,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn new(db: Arc<Mutex<db::MinebotDB>>, data: AppData) -> Self {
        App {
            mode: Mode::default(),
            db,
            db_timer: timers::Timer::from_seconds(5.0),
            stopwatch: timers::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            data,
        }
    }

    pub fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    pub fn tick(&mut self) -> Result<()> {
        // calculate delta time
        let now = Instant::now();
        self.dt = now.duration_since(self.last_time);
        self.last_time = now;

        // update app timers
        self.db_timer.tick(self.dt);
        self.stopwatch.tick(self.dt);
        self.data.bot_ops.tick(self.dt);

        // query db for new bot operations
        if self.db_timer.finished() {
            if let Some(db) = self.db.lock().ok() {
                let mut stmt = db.conn.prepare("SELECT pubkey, state, data FROM bot_ops")?;
                let bot_ops_iter = stmt.query_map([], |row| {
                    let pubkey: String = row.get(0)?;
                    let state: String = row.get(1)?;
                    let data: String = row.get(2)?;

                    let operation: Option<actors::BotOps> = serde_json::from_str(&data).ok();
                    Ok((pubkey, state, operation))
                })?;

                let bot_ops: Vec<_> = bot_ops_iter.filter_map(Result::ok).collect();
                // self.data.bot_ops = ui::BotOpsUI::from(bot_ops);
                self.data.bot_ops.update(&bot_ops);

                self.db_timer.reset();
            };
        }

        Ok(())
    }

    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}
