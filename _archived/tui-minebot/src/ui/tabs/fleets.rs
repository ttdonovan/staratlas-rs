use anchor_client::anchor_lang::prelude::Pubkey;
use ratatui::{prelude::*, widgets::*};

use std::collections::HashMap;

use crate::bots;

pub struct FleetsTab<'a> {
    bots: &'a HashMap<Pubkey, bots::MiningBot>,
}

impl<'a> FleetsTab<'a> {
    pub fn new(bots: &'a HashMap<Pubkey, bots::MiningBot>) -> Self {
        FleetsTab { bots }
    }
}

impl<'a> Widget for FleetsTab<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
        tx_table.set_header(vec!["Fleet ID", "Last Txs", "Counter", "Errors", "Is Tx"]);

        for (_, bot) in self.bots {
            table.add_row(vec![
                format!("{}", bot.masked_fleet_id()),
                format!("{}", bot.fleet_name()),
                format!("{}", bot.mine_item_name),
                format!("{}", bot.mine_rate()),
                format!("{}", bot.mine_amount()),
                format!("{:.2}s", bot.mine_duration().as_secs_f32()),
                format!(
                    "{:.2}s ({:.3})",
                    bot.mining_timer.remaining_secs(),
                    bot.mining_timer.fraction()
                ),
                format!("{}/{}", bot.fuel_tank_amount, bot.fuel_tank_capacity),
                format!("{}/{}", bot.ammo_bank_amount, bot.ammo_bank_capacity),
                format!("{}/{}", bot.cargo_hold_amount, bot.cargo_hold_capacity),
                format!("{:#?}", bot.autoplay),
            ]);

            tx_table.add_row(vec![
                format!("{}", bot.masked_fleet_id()),
                format!("{}", bot.last_txs()),
                format!("{}", bot.txs_counter),
                format!("{}", bot.txs_errors),
                format!("{}", bot.is_tx),
            ]);
        }

        Paragraph::new(Text::raw(format!("{table}\n{tx_table}"))).render(area, buf);
    }
}
