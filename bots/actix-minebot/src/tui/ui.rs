use anchor_client::anchor_lang::prelude::Pubkey;
use comfy_table::Table;

use staratlas_sage_based_sdk::{Fleet, Game};

fn ui_pubkey(pubkey: &Pubkey) -> String {
    let id = pubkey.to_string();
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
