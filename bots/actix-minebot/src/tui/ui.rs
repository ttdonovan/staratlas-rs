use anchor_client::anchor_lang::prelude::Pubkey;
use comfy_table::Table;

use staratlas_sage_based_sdk::{Fleet, Game};

use std::time::Duration;

use crate::{actors, timers};

fn ui_pubkey(pubkey: &Pubkey) -> String {
    let id = pubkey.to_string();
    format!("{}...{}", &id[..4], &id[40..])
}

fn ui_pubkey_str(pubkey_str: &str) -> String {
    let id = pubkey_str;
    format!("{}...{}", &id[..4], &id[40..])
}

pub(crate) struct GameUI(pub Pubkey, pub Game);

impl From<(Pubkey, Game)> for GameUI {
    fn from(game: (Pubkey, Game)) -> Self {
        GameUI(game.0, game.1)
    }
}

impl GameUI {
    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table.set_header(vec!["Game State", "Points", "Cargo", "Mints"]);
        table.add_row(vec![
            format!("{:#?}", self.1.game_state),
            format!("{:#?}", self.1.points),
            format!("{:#?}", self.1.cargo),
            format!("{:#?}", self.1.mints),
        ]);

        table
    }
}

pub(crate) struct FleetsUI(pub Vec<(Pubkey, Fleet)>);

impl From<Vec<(Pubkey, Fleet)>> for FleetsUI {
    fn from(fleets: Vec<(Pubkey, Fleet)>) -> Self {
        FleetsUI(fleets)
    }
}

impl FleetsUI {
    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table.set_header(vec![
            "Fleet ID",
            "Name/Callsign",
            "Ship Counts",
            "Movement Stats",
            "Cargo Stats",
            "Misc Stats",
            "Addresses",
        ]);

        for (pubkey, fleet) in self.0.iter() {
            let address = serde_json::json!({
                "cargo_hold": fleet.cargo_hold.to_string(),
                "fuel_tank": fleet.fuel_tank.to_string(),
                "ammo_bank": fleet.ammo_bank.to_string(),
            });

            table.add_row(vec![
                format!("{}", ui_pubkey(pubkey)),
                format!("{}", fleet.name()),
                format!("{:#?}", fleet.ship_counts),
                format!("{:#?}", fleet.stats.movement_stats),
                format!("{:#?}", fleet.stats.cargo_stats),
                format!("{:#?}", fleet.stats.misc_stats),
                format!("{:#?}", address),
            ]);
        }

        table
    }
}

pub(crate) struct BotOpsUI(Vec<(String, Option<actors::BotOps>, BotOpsData)>);

#[derive(Default)]
pub(crate) struct BotOpsData {
    mining_location: String,
    currently_mining: String,
    timer: Option<timers::Timer>,
    stopwatch: Option<timers::Stopwatch>,
}

impl From<Vec<(String, Option<actors::BotOps>)>> for BotOpsUI {
    fn from(bot_ops: Vec<(String, Option<actors::BotOps>)>) -> Self {
        let bot_ops = bot_ops
            .into_iter()
            .map(|(pubkey_str, operation)| (pubkey_str, operation, BotOpsData::default()))
            .collect();

        BotOpsUI(bot_ops)
    }
}

impl BotOpsUI {
    pub fn tick(&mut self, dt: Duration) {
        for (_, _, data) in self.0.iter_mut() {
            if let Some(stopwatch) = &mut data.stopwatch {
                stopwatch.tick(dt);
            }

            if let Some(timer) = &mut data.timer {
                timer.tick(dt);
            }
        }
    }

    pub fn update(&mut self, ops: &[(String, Option<actors::BotOps>)]) {
        for (pubkey_str, operation) in ops {
            let entry = self.0.iter_mut().find(|(key, _, _)| key == pubkey_str);

            match entry {
                Some((_, ops, data)) => {
                    match operation {
                        Some(actors::BotOps::Idle(o)) => {
                            data.stopwatch = Some(o.stopwatch);
                            data.timer = None;
                        }
                        Some(actors::BotOps::Mining(o)) => {
                            data.mining_location = o.mining_location.to_string();
                            data.currently_mining = o.currently_mining.to_string();
                            data.timer = Some(o.timer);
                        }
                        Some(actors::BotOps::StarbaseLoadingBay(o)) => {
                            data.stopwatch = Some(o.stopwatch);
                            data.timer = None;
                        }
                        Some(actors::BotOps::TxsSageBased(o)) => {
                            data.stopwatch = Some(o.stopwatch);
                            data.timer = None;
                        }
                        _ => {
                            data.stopwatch = None;
                            data.timer = None;
                        }
                    }

                    *ops = operation.clone();
                }
                _ => {
                    self.0.push((
                        pubkey_str.to_string(),
                        operation.clone(),
                        BotOpsData::default(),
                    ));
                }
            }
        }
    }

    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table.set_header(vec![
            "Fleet ID",
            "Mining Location",
            "Currently Mining",
            "dbg!",
            "Timer",
            "Stopwatch",
        ]);

        for (pubkey_str, bot_ops, data) in self.0.iter() {
            table.add_row(vec![
                format!("{}", ui_pubkey_str(pubkey_str)),
                format!("{}", data.mining_location),
                format!("{}", data.currently_mining),
                format!("{:#?}", bot_ops),
                format!("{:#?}", data.timer),
                format!("{:#?}", data.stopwatch),
            ]);
        }

        table
    }
}
