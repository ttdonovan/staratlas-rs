use actix::prelude::*;
use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    Client,
};
use color_eyre::Result;
use tokio::time;

use staratlas_sage_based_sdk::{program::SAGE_ID, state, Game, SageBasedGameHandler};

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
        let planet_id = Pubkey::from_str(&bot_cfg.planet_id).unwrap();
        let mine_item_id = Pubkey::from_str(&bot_cfg.mine_item_id).unwrap();

        let (fleet_id, fleet_with_state) =
            SageBasedGameHandler::get_fleet_with_state(&program, &fleet_id).await?;

        let mine_item = SageBasedGameHandler::get_mine_item(&program, &mine_item_id).await?;
        let planet = SageBasedGameHandler::get_planet(&program, &planet_id).await?;
        let resource =
            SageBasedGameHandler::find_resource(&program, &game_id, &planet_id, &mine_item_id)
                .await?;

        // create a new bot actor
        let bot_addr = actors::BotActor::new(
            (fleet_id, fleet_with_state.fleet),
            planet,
            mine_item,
            resource,
            sage_addr.clone(),
            db.clone(),
        )
        .start();

        // warm-up the bot actor
        {
            sage_addr
                .send(actors::SubscribeClockTime(bot_addr.clone().recipient()))
                .await?;

            sage_addr
                .send(actors::SageRequest::Fleet(fleet_id, bot_addr.clone()))
                .await?;
        }

        bot_addrs.push(bot_addr);
        fleets.push((fleet_id, fleet_with_state.fleet));
    }

    // request the current clock time
    sage_addr.send(actors::ClockTime).await?;

    // wait a few seconds as bot actors warm-up and initalize state
    // FIXME: if not waiting long enough, the bot actors will panic
    tokio::time::sleep(time::Duration::from_secs(5)).await;

    let mut interval = time::interval(time::Duration::from_secs(10));
    let mut delta = time::Instant::now();

    let terminal = &mut term::init()?;

    let app = app::init(db, (game_id, game), fleets);
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
