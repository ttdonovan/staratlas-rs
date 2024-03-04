use clap::Parser;

use shared_sage_context as sage;
use shared_time as time;

mod app;
mod bots;
mod cli;
mod errors;
mod term;

fn main() -> anyhow::Result<()> {
    errors::init_hooks().unwrap();
    tui_logger::init_logger(log::LevelFilter::Info)?;
    tui_logger::set_default_level(log::LevelFilter::Info);
    log::info!(target: "App", "logging initialized");

    let cli = cli::Cli::parse();
    let client = cli::init_client(&cli)?;
    let (game_id, fleet_ids) = cli::init_sage_config(&cli);

    let sage = sage::SageContext::new(&client, &game_id)?;
    let bots = bots::init_bots(&sage, fleet_ids)?;

    let terminal = &mut term::init()?;
    app::run(sage, bots, terminal)?;
    term::restore()?;
    Ok(())
}
