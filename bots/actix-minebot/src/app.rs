use anchor_client::anchor_lang::prelude::Pubkey;
use staratlas_sage_based_sdk::{Fleet, Game};

use std::time::{Duration, Instant};

use crate::{timers, tui::ui};

pub fn init(game: (Pubkey, Game), fleets: Vec<(Pubkey, Fleet)>) -> App {
    App::new(AppData {
        game_ui: game.into(),
        fleets_ui: fleets.into(),
    })
}

pub struct App {
    mode: Mode,
    pub(crate) stopwatch: timers::Stopwatch,
    dt: Duration,
    last_time: Instant,
    pub(crate) data: AppData,
}

pub struct AppData {
    pub(crate) game_ui: ui::GameUI,
    pub(crate) fleets_ui: ui::FleetsUI,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn new(data: AppData) -> Self {
        App {
            mode: Mode::default(),
            stopwatch: timers::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            data,
        }
    }

    pub fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    pub fn tick(&mut self) {
        // calculate delta time
        let now = Instant::now();
        self.dt = now.duration_since(self.last_time);
        self.last_time = now;

        // update app timers
        self.stopwatch.tick(self.dt);
    }

    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}
