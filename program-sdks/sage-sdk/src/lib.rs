pub use staratlas_sage::{program, state};

use std::fmt;
use std::str;

pub mod fleets;
pub mod games;
pub mod utils;

pub struct Fleet(state::Fleet);

impl Fleet {
    pub fn fleet_label(&self) -> &str {
        let fleet_label = str::from_utf8(&self.0.fleet_label).unwrap();
        let fleet_lable_trimmed = fleet_label.trim_end_matches(char::from(0));
        fleet_lable_trimmed
    }
}

impl fmt::Debug for Fleet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fleet")
            .field("vesion", &self.0.version)
            .field("game_id", &self.0.game_id)
            .field("owner_profile", &self.0.owner_profile)
            .field("fleet_ships", &self.0.fleet_ships)
            .field("fleet_label", &self.fleet_label())
            .field("ship_counts", &self.0.ship_counts)
            .field("stats", &self.0.stats)
            .field("cargo_hold", &self.0.cargo_hold)
            .field("fuel_tank", &self.0.fuel_tank)
            .field("ammo_bank", &self.0.ammo_bank)
            .field("update_id", &self.0.update_id)
            .finish()
    }
}

pub struct Game(state::Game);

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("vesion", &self.0.version)
            .field("update_id", &self.0.update_id)
            .field("profile", &self.0.profile)
            .field("game_state", &self.0.game_state)
            .finish()
    }
}

pub struct GameState(state::GameState);

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameState")
            .field("vesion", &self.0.version)
            .field("update_id", &self.0.update_id)
            .field("game_id", &self.0.game_id)
            .finish()
    }
}
