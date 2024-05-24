use anchor_client::anchor_lang::prelude::Pubkey;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::{bots, labs, time};

pub struct App {
    mode: Mode,
    pub stopwatch: time::Stopwatch,
    dt: Duration,
    last_time: Instant,
    sage: labs::SageLabsHandler,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

pub fn init(game_id: Pubkey, fleet_ids: Vec<Pubkey>) -> anyhow::Result<App> {
    let app = App::new(game_id, fleet_ids);
    app.sage.refresh_fleet()?;

    Ok(app)
}

impl App {
    pub fn new(game_id: Pubkey, fleet_ids: Vec<Pubkey>) -> Self {
        let sage = labs::SageLabsHandler::new(game_id.clone(), fleet_ids.clone());

        App {
            mode: Mode::default(),
            stopwatch: time::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            sage,
        }
    }

    pub fn bots(&self) -> &HashMap<Pubkey, bots::MiningBot> {
        &self.sage.bots
    }

    pub fn game_id(&self) -> &Pubkey {
        &self.sage.game_id
    }

    pub fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        // calculate delta time
        let now = Instant::now();
        self.dt = now.duration_since(self.last_time);
        self.last_time = now;

        // update app timers
        self.stopwatch.tick(self.dt);

        // for each SAGE Labs bot run update
        self.sage.run_bots(self.dt)?;

        // poll for responses from SAGE Labs
        self.sage.poll_response()?;

        Ok(())
    }

    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}
