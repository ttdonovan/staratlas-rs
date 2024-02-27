use clap::Parser;

use shared_time as time;

mod app;
mod cli;
mod errors;
mod sage;
mod term;

fn main() -> anyhow::Result<()> {
    errors::init_hooks().unwrap();
    let cli = cli::Cli::parse();
    let client = cli::init_client(&cli)?;

    let mut games = sage::list_games(&client)?;
    let game = games.remove(0);

    let mut profiles = sage::list_player_profiles(&client)?;
    let profile = profiles.remove(0);

    let sage = sage::SageContext::new(game, profile, client)?;

    let terminal = &mut term::init()?;
    app::run(sage, terminal)?;
    term::restore()?;
    Ok(())
}
