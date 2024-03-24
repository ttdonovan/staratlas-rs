use crate::{sage, time, Pubkey};

use std::time::Duration;

pub type BotArgs = ([i64; 2], [i64; 2], Pubkey, usize);

#[derive(Debug)]
pub struct Bot {
    pub fleet_id: Pubkey,
    pub fleet: sage::Fleet,
    pub fleet_state: sage::FleetState,
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub mint: Pubkey,
    pub timers: BotTimers,
    pub num_runs: usize,
    pub is_tx: bool,
}

impl Bot {
    pub fn is_warp_cool_down_finished(&self) -> bool {
        self.timers.warp_cool_down.finished()
    }

    pub fn warp_cool_down_remaining_secs(&self) -> f32 {
        self.timers.warp_cool_down.remaining_secs()
    }
}

#[derive(Debug)]
pub struct BotTimers {
    pub warp_cool_down: time::Timer,
}

impl BotTimers {
    pub fn tick(&mut self, dt: Duration) {
        self.warp_cool_down.tick(dt);
    }
}
