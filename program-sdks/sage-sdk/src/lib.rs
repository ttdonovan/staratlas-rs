use std::fmt;
use std::str;

pub mod derive;
pub mod find;
pub mod ixs;
pub mod programs;
pub mod utils;

use programs::staratlas_cargo;
use programs::staratlas_sage::{state, typedefs};

#[derive(Clone, Copy)]
pub struct CargoPod(staratlas_cargo::state::CargoPod);

impl fmt::Debug for CargoPod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CargoPod")
            .field("version", &self.0.version)
            .field("stats_definition", &self.0.stats_definition)
            .field("authority", &self.0.authority)
            .finish()
    }
}

pub struct CargoStatsDefinition(staratlas_cargo::state::CargoStatsDefinition);

impl fmt::Debug for CargoStatsDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CargoStatsDefinition")
            .field("version", &self.0.version)
            .field("authority", &self.0.authority)
            .field("default_cargo_type", &self.0.default_cargo_type)
            .field("stats_count", &self.0.stats_count)
            .field("seq_id", &self.0.seq_id)
            .finish()
    }
}

pub struct CargoType(staratlas_cargo::state::CargoType);

impl fmt::Debug for CargoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CargoType")
            .field("version", &self.0.version)
            .field("stats_definition", &self.0.stats_definition)
            .field("mint", &self.0.mint)
            .field("bump", &self.0.bump)
            .field("stats_count", &self.0.stats_count)
            .field("seq_id", &self.0.seq_id)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct Fleet(pub state::Fleet);

impl Fleet {
    pub fn fleet_label(&self) -> &str {
        let fleet_label = str::from_utf8(&self.0.fleet_label).unwrap();
        let fleet_label_trimmed = fleet_label.trim_end_matches(char::from(0));
        fleet_label_trimmed
    }
}

impl fmt::Debug for Fleet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fleet")
            .field("version", &self.0.version)
            .field("game_id", &self.0.game_id)
            .field("owner_profile", &self.0.owner_profile)
            .field("fleet_ships", &self.0.fleet_ships)
            // .field("fleet_label", &self.fleet_label())
            .field("ship_counts", &self.0.ship_counts)
            .field("stats", &self.0.stats)
            .field("cargo_hold", &self.0.cargo_hold)
            .field("fuel_tank", &self.0.fuel_tank)
            .field("ammo_bank", &self.0.ammo_bank)
            .field("update_id", &self.0.update_id)
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FleetState {
    StarbaseLoadingBay(typedefs::StarbaseLoadingBay),
    Idle(typedefs::Idle),
    MineAsteroid(typedefs::MineAsteroid),
    MoveWarp(typedefs::MoveWarp),
    MoveSubwarp(typedefs::MoveSubwarp),
    Respawn(typedefs::Respawn),
}

pub struct Game(pub state::Game);

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Game")
            .field("version", &self.0.version)
            .field("update_id", &self.0.update_id)
            .field("profile", &self.0.profile)
            .field("game_state", &self.0.game_state)
            .field("cargo", &self.0.cargo)
            .field("mints", &self.0.mints)
            .finish()
    }
}

pub struct GameState(state::GameState);

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameState")
            .field("version", &self.0.version)
            .field("update_id", &self.0.update_id)
            .field("game_id", &self.0.game_id)
            .finish()
    }
}

pub struct Planet(pub state::Planet);

impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Planet")
            .field("version", &self.0.version)
            .field("game_id", &self.0.game_id)
            .field("sector", &self.0.sector)
            .field("sub_coordinates", &self.0.sub_coordinates)
            .field("planet_type", &self.0.planet_type)
            .field("position", &self.0.position)
            .field("size", &self.0.size)
            .field("max_hp", &self.0.max_hp)
            .field("current_health", &self.0.current_health)
            .field("amount_mined", &self.0.amount_mined)
            .field("num_resources", &self.0.num_resources)
            .field("num_miners", &self.0.num_miners)
            .finish()
    }
}

pub struct MineItem(pub state::MineItem);

impl fmt::Debug for MineItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MineItem")
            .field("version", &self.0.version)
            .field("game_id", &self.0.game_id)
            // .filed("name", &self.0.name)
            .field("mint", &self.0.mint)
            .field("mine_item_update_id", &self.0.mine_item_update_id)
            .field("resource_hardness", &self.0.resource_hardness)
            .field("num_resource_accounts", &self.0.num_resource_accounts)
            .field("bump", &self.0.bump)
            .finish()
    }
}

impl MineItem {
    pub fn name(&self) -> &str {
        let name = str::from_utf8(&self.0.name).unwrap();
        let name_trimmed = name.trim_end_matches(char::from(0));
        name_trimmed
    }
}
pub struct Resource(pub state::Resource);

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resource")
            .field("version", &self.0.version)
            .field("game_id", &self.0.game_id)
            .field("location", &self.0.location)
            .field("mine_item", &self.0.mine_item)
            .field("location_type", &self.0.location_type)
            .field("system_richness", &self.0.system_richness)
            .field("amount_mined", &self.0.amount_mined)
            .field("num_miners", &self.0.num_miners)
            .field("mine_item_update_id", &self.0.mine_item_update_id)
            .field("resource_update_id", &self.0.resource_update_id)
            .field("bump", &self.0.bump)
            .finish()
    }
}

pub struct Starbase(pub state::Starbase);

impl fmt::Debug for Starbase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Starbase")
            .field("version", &self.0.version)
            .field("game_id", &self.0.game_id)
            .field("sector", &self.0.sector)
            .field("crafting_facility", &self.0.crafting_facility)
            .field("seq_id", &self.0.seq_id)
            .finish()
    }
}
