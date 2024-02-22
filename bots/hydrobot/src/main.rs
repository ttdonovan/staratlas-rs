use anchor_client::solana_sdk::pubkey::Pubkey;
use clap::Parser;
use solana_program::pubkey;

mod app;
mod bot;
mod cli;
mod errors;
mod sage;
mod traits;
mod ui;

const HYDRO_MINT: Pubkey = pubkey!("HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp");

#[macroquad::main("Hydrobot")]
async fn main() -> anyhow::Result<()> {
    errors::init_hooks().unwrap();
    let cli = cli::Cli::parse();

    // Setup payer, client and sage config from cli
    let (_payer, client) = cli::init_payer_and_client(&cli)?;
    let (game_id, fleet_ids) = cli::init_sage_config(&cli);

    // Setup sage game handler
    let game_handler = sage::init_game_handler(&client, &game_id)?;

    // Setup sage bots from game handler and fleet id
    let mut bots = Vec::new();
    for fleet_id in &fleet_ids {
        let mut bot = bot::init(&game_handler, fleet_id, &HYDRO_MINT)?;
        let autoplay = cli.autoplay_enabled();
        bot.set_autoplay(autoplay);
        bots.push(bot);
    }

    // Run the bots in a game loop
    app::run(&game_handler, bots).await?;

    Ok(())
}
