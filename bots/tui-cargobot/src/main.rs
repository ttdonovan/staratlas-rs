use anchor_lang::prelude::Pubkey;

use shared_sage_cli as cli;
use shared_sage_context as sage;
use shared_time as time;

use std::str::FromStr;

mod app;
mod bots;
mod errors;
mod labs;
mod term;
mod tui;

// FIXME: temporary arguments for development/testing
const MAX_CARGO_AMOUNT: u64 = 1000;
const MAX_CARGO_RUNS: usize = 3;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    errors::init_hooks().unwrap();
    tui_logger::init_logger(log::LevelFilter::Info)?;
    tui_logger::set_default_level(log::LevelFilter::Info);
    log::info!("[Logger] - logging initialized");

    let cli = cli::cli_parse();
    let client = cli::init_client(&cli)?;

    let (game_id, fleet_ids) = cli::init_sage_config(&cli);
    // Only 1 fleet at this time...
    let fleet_id = fleet_ids[0];

    // Biomass From/To: CSS [40, 30] and UST-2 [42, 35]
    // Carbon From/To: CSS [40, 30] and UST-3 [48, 32]
    let from_sector = [40, 30];
    let to_sector = [42, 35];

    // Biomass: MASS9GqtJz6ABisAxcUn3FeR4phMqH1XfG6LPKJePog
    // Carbon: CARBWKWvxEuMcq3MqCxYfi7UoFVpL9c4rsQS99tw6i4X
    let mint = Pubkey::from_str("MASS9GqtJz6ABisAxcUn3FeR4phMqH1XfG6LPKJePog")?;

    let args: bots::BotArgs = (from_sector, to_sector, mint, 0);
    let app = app::init(game_id, fleet_id, args)?;

    let terminal = &mut term::init()?;
    tui::run(app, terminal).await?;
    term::restore()?;

    Ok(())
}
