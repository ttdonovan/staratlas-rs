use actix::prelude::*;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    Client,
};
use color_eyre::Result;
use tokio::time;

use staratlas_sage_based_sdk::{addr, program::SAGE_ID, state, Game, SageBasedGameHandler};

use std::str::FromStr;
use std::sync::{Arc, Mutex};

use shared_time as timers;

mod actors;
mod app;
mod config;
mod db;
mod errors;
mod term;
mod tui;

#[actix::main]
async fn main() -> Result<()> {
    errors::init_hooks()?;
    config::init_logger()?;

    // initialize the configuration (includes hot wallet's payer/keyair)
    let cfg = config::init_config()?;
    let payer = cfg.payer;
    let game_id = Pubkey::from_str(&cfg.sage_bot_cfg.game_id).unwrap();

    // create a new client and program
    let client =
        Client::new_with_options(cfg.cluster, payer.clone(), CommitmentConfig::confirmed());
    let program = client.program(SAGE_ID)?;

    let account = program.account::<state::Game>(game_id).await?;
    let game = Game::from(account);

    // create a new Sage Based actor (take "ownership" of the client, payer, game_id, and game)
    let sage_addr = actors::SageBasedActor::new(client, payer, game_id, game).start();
    sage_addr.send(actors::BlockHeight).await?;

    let mut bot_addrs = vec![];
    let mut fleets = vec![];

    // in-memory database for bot operations
    let db = db::MinebotDB::open()?;
    let db = Arc::new(Mutex::new(db));

    for bot_cfg in &cfg.sage_bot_cfg.bots {
        let fleet_id = Pubkey::from_str(&bot_cfg.fleet_id).unwrap();

        let (fleet_id, fleet_with_state) =
            SageBasedGameHandler::get_fleet_with_state(&program, &fleet_id).await?;

        // create a role assignment for the bot
        let role = match &bot_cfg.role.0 {
            config::BotRoleArgs::MineAsteroid {
                planet_id,
                mine_item_id,
            } => {
                let planet_id = Pubkey::from_str(planet_id).unwrap();
                let mine_item_id = Pubkey::from_str(mine_item_id).unwrap();

                let mine_item =
                    SageBasedGameHandler::get_mine_item(&program, &mine_item_id).await?;
                let planet = SageBasedGameHandler::get_planet(&program, &planet_id).await?;
                let resource = SageBasedGameHandler::find_resource(
                    &program,
                    &game_id,
                    &planet_id,
                    &mine_item_id,
                )
                .await?;

                actors::BotRole::MineAsteroid {
                    planet,
                    mine_item,
                    resource,
                }
            }
            config::BotRoleArgs::CargoTransport {
                cargo_mint,
                cargo_amount,
                from_sector,
                to_sector,
            } => {
                let (from_starbase, _) = addr::starbase_address(&game_id, *from_sector);
                let (to_starbase, _) = addr::starbase_address(&game_id, *to_sector);

                actors::BotRole::CargoTransport {
                    cargo_mint: Pubkey::from_str(cargo_mint).unwrap(),
                    cargo_amount: *cargo_amount,
                    from_sector: *from_sector,
                    from_starbase,
                    to_sector: *to_sector,
                    to_starbase,
                }
            }
        };

        // create a new bot actor
        let bot_addr = actors::BotActor::new(
            db.clone(),
            sage_addr.clone(),
            (fleet_id, fleet_with_state.clone()),
            role,
        )
        .start();

        // subscribe to the clock time
        sage_addr
            .send(actors::SubscribeClockTime(bot_addr.clone().recipient()))
            .await?;

        bot_addrs.push(bot_addr);
        fleets.push((fleet_id, fleet_with_state.0));
    }

    // request the current clock time to kick-off the bot actors
    sage_addr.send(actors::ClockTime).await?;

    let mut interval = time::interval(time::Duration::from_secs(10));
    let mut delta = time::Instant::now();

    let app = app::init(db, (game_id, game), fleets);
    let terminal = &mut term::init()?;
    let mut tui = tui::init(app);

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let dt = delta.elapsed();

                // send tick with delta-time to all bot actors
                for addr in &bot_addrs {
                    addr.send(actors::Tick(dt)).await?;
                }

                delta = time::Instant::now();
            }
            _ = tui.run(terminal) => {
                break;
            }
            _ = tokio::signal::ctrl_c() => {
                break;
            }
        }
    }

    System::current().stop();
    term::restore()?;

    Ok(())
}
