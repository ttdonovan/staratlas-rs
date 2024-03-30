use shared_sage_cli as cli;
use shared_sage_context as sage;
use shared_time as time;

mod app;
mod bots;
mod errors;
mod labs;
mod term;
mod tui;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    errors::init_hooks().unwrap();
    tui_logger::init_logger(log::LevelFilter::Info)?;
    tui_logger::set_default_level(log::LevelFilter::Info);
    log::info!(target: "App", "logging initialized");

    let cli = cli::cli_parse();
    let (game_id, fleet_ids) = cli::init_sage_config(&cli);

    let app = app::init(game_id, fleet_ids)?;
    let terminal = &mut term::init()?;
    tui::run(app, terminal).await?;
    term::restore()?;

    Ok(())
}
