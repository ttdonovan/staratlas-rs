use std::sync::RwLock;
use std::time::{Duration, Instant};

use crate::{bots, sage, time, txs};

pub struct App {
    mode: Mode,
    pub stopwatch: time::Stopwatch,
    dt: Duration,
    last_time: Instant,
    pub sage_labs: txs::SageLabs,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

pub fn init(context: sage::SageContext, bots: Vec<bots::MiningBot>) -> App {
    App::new(context, bots)
}

impl App {
    pub fn new(context: sage::SageContext, bots: Vec<bots::MiningBot>) -> Self {
        let sage_labs = txs::SageLabs::new(context, bots);

        App {
            mode: Mode::default(),
            stopwatch: time::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            sage_labs,
        }
    }

    pub fn bots(&self) -> &Vec<RwLock<bots::MiningBot>> {
        self.sage_labs.bots.as_ref()
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
        self.sage_labs.bots_run_update(self.dt)?;

        Ok(())
    }

    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}
