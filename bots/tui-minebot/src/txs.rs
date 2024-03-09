use anchor_client::solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::{bots, sage};

pub fn sage_start_mining_asteroid(
    bot: &mut bots::MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.start_mining_asteroid(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;

            Ok(signature)
        }
        Err(err) => {
            log::error!(
                "[{}] Start Mining Asteroid: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_stop_mining_asteroid(
    bot: &mut bots::MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.stop_mining_asteroid(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;

            Ok(signature)
        }
        Err(err) => {
            log::error!(
                "[{}] Stop Mining Asteroid: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_dock_to_starbase(
    bot: &mut bots::MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.dock_to_starbase(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            Ok(signature)
        }
        Err(err) => {
            log::error!("[{}] Dock to Starbase: {:?}", bot.masked_fleet_id(), err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_undock_from_starbase(
    bot: &mut bots::MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.undock_from_starbase(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            Ok(signature)
        }
        Err(err) => {
            log::error!(
                "[{}] Undock from Starbase: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_mine_item_widthdraw_from_fleet(
    bot: &mut bots::MiningBot,
    sage: &sage::SageContext,
) -> Result<Option<Signature>, anyhow::Error> {
    let (fleet_id, _, _, fleet, _) = &bot.fleet;
    let starbase = bot.starbase_id().unwrap();
    let mine_item = &bot.mine_item.2;
    let mint = &mine_item.0.mint;

    match sage.withdraw_from_fleet(
        fleet_id, fleet, starbase, mint, None, // withdraw all (max)
    ) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;

            if let Some(signature) = signature {
                bot.txs = Some(signature);
                bot.txs_counter += 1;
                return Ok(Some(signature));
            }

            Ok(None)
        }
        Err(err) => {
            log::error!(
                "[{}], Widthdraw from Fleet: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_deposit_to_fleet(
    bot: &mut bots::MiningBot,
    cargo_pod_to: &Pubkey,
    mint: &Pubkey,
    amount: u64,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, _) = &bot.fleet;
    let starbase = bot.starbase_id().unwrap();

    match sage.deposit_to_fleet(fleet_id, fleet, starbase, cargo_pod_to, mint, amount) {
        Ok(signature) => {
            bot.txs = Some(signature);
            bot.txs_counter += 1;

            bot.is_fleet_state_dirty = true;
            Ok(signature)
        }
        Err(err) => {
            log::error!("[{}] Deposit to Fleet: {:?}", bot.masked_fleet_id(), err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub enum Event {
    StartMiningAsteroid(usize),
    StopMiningAsteroid(usize),
    DockToStarbase(usize),
    UndockFromStarbase(usize),
    StarbaseHangarCargoWithdraw(usize),
    StarbaseHangarDepositToFleet(usize, bots::CargoDeposit),
}

pub struct SageLabs {
    pub ctx: sage::SageContext,
    pub bots: Arc<Vec<RwLock<bots::MiningBot>>>,
    tx: mpsc::Sender<Event>,
    // _task: JoinHandle<()>,
}

impl SageLabs {
    pub fn new(ctx: sage::SageContext, bots: Vec<bots::MiningBot>) -> Self {
        let (tx, rx) = mpsc::channel::<Event>();
        let bots: Arc<Vec<RwLock<bots::MiningBot>>> =
            Arc::new(bots.into_iter().map(RwLock::new).collect());
        let bots_clone = Arc::clone(&bots);

        // let _task = thread::spawn(move || {
        //     use crate::cli;
        //     use clap::Parser;

        //     // this 'task' thread needs it's own client and sage context
        //     let cli = cli::Cli::parse();
        //     let client = cli::init_client(&cli).unwrap();
        //     let (game_id, _) = cli::init_sage_config(&cli);
        //     let sage = sage::SageContext::new(&client, &game_id).unwrap();

        //     let bots = bots.as_ref();
        //     while let Some(event) = rx.recv().ok() {
        //         match event {
        //             Event::StartMiningAsteroid(index) => {
        //                 let mut bot = bots[index].write().unwrap();

        //                 log::info!(
        //                     "SageLabs > Start Mining Asteroid ({}): {}",
        //                     bot.masked_fleet_id(),
        //                     &ctx.game_id
        //                 );

        //                 if sage_start_mining_asteroid(&mut bot, &sage).is_ok() {
        //                     bot.autoplay = bots::Autoplay::IsMiningAsteroid;
        //                     bot.is_tx = false;

        //                     // start our mining timer
        //                     bot.reset_mining_timer();
        //                 };
        //             }
        //             Event::StopMiningAsteroid(index) => {
        //                 let mut bot = bots[index].write().unwrap();

        //                 log::info!(
        //                     "SageLabs > Stop Mining Asteroid ({}): {}",
        //                     bot.masked_fleet_id(),
        //                     &ctx.game_id
        //                 );

        //                 if sage_stop_mining_asteroid(&mut bot, &sage).is_ok() {
        //                     // FIXME: ideally the fleet would go back to idle and all logic is correct there...
        //                     bot.autoplay = bots::Autoplay::StarbaseDock;
        //                     bot.is_tx = false;
        //                 };
        //             }
        //             Event::DockToStarbase(index) => {
        //                 let mut bot = bots[index].write().unwrap();

        //                 log::info!(
        //                     "SageLabs > Dock to Starbase ({}): {}",
        //                     bot.masked_fleet_id(),
        //                     &ctx.game_id
        //                 );

        //                 if sage_dock_to_starbase(&mut bot, &sage).is_ok() {
        //                     bot.autoplay = bots::Autoplay::StarbaseHangarCargoWithdraw;
        //                     bot.is_tx = false;
        //                 };
        //             }
        //             Event::UndockFromStarbase(index) => {
        //                 let mut bot = bots[index].write().unwrap();

        //                 log::info!(
        //                     "SageLabs > Undock from Starbase ({}): {}",
        //                     bot.masked_fleet_id(),
        //                     &ctx.game_id
        //                 );

        //                 if sage_undock_from_starbase(&mut bot, &sage).is_ok() {
        //                     bot.autoplay = bots::Autoplay::IsIdle;
        //                     bot.is_tx = false;
        //                 };
        //             }
        //             Event::StarbaseHangarCargoWithdraw(index) => {
        //                 let mut bot = bots[index].write().unwrap();

        //                 log::info!(
        //                     "SageLabs > Starbase Hangar Cargo Withdraw ({}): {}",
        //                     bot.masked_fleet_id(),
        //                     &ctx.game_id
        //                 );

        //                 if sage_mine_item_widthdraw_from_fleet(&mut bot, &sage).is_ok() {
        //                     bot.autoplay = bots::Autoplay::StarbaseHangarCargoDeposit(
        //                         bots::CargoDeposit::Fuel,
        //                     );
        //                     bot.is_tx = false;
        //                 };
        //             }
        //             Event::StarbaseHangarDepositToFleet(index, deposit) => {
        //                 let mut bot = bots[index].write().unwrap();

        //                 log::info!(
        //                     "SageLabs > Starbase Hangar Deposit to Fleet ({}): {:?}",
        //                     bot.masked_fleet_id(),
        //                     deposit
        //                 );

        //                 let (cargo_pod_to, mint, amount) = match deposit {
        //                     bots::CargoDeposit::Fuel => {
        //                         let (fuel_tank, actual, capacity) = bot.fuel_tank.clone();
        //                         let fuel_mint = &sage.game_acct.0.mints.fuel;
        //                         let amount = (capacity - actual) as u64;

        //                         (fuel_tank, fuel_mint, amount)
        //                     }
        //                     bots::CargoDeposit::Ammo => {
        //                         let (ammo_bank, actual, capacity) = bot.ammo_bank.clone();
        //                         let ammo_mint = &sage.game_acct.0.mints.ammo;
        //                         let amount = (capacity - actual) as u64;

        //                         (ammo_bank, ammo_mint, amount)
        //                     }
        //                     bots::CargoDeposit::Food => {
        //                         let (cargo_hold, _actual, capacity) = bot.cargo_hold.clone();
        //                         let food_mint = &sage.game_acct.0.mints.food;
        //                         let amount = (capacity as f32 * 0.05) as u64;

        //                         (cargo_hold, food_mint, amount)
        //                     }
        //                 };

        //                 if sage_deposit_to_fleet(&mut bot, &cargo_pod_to, mint, amount, &sage)
        //                     .is_ok()
        //                 {
        //                     bot.autoplay = bots::Autoplay::StarbaseHangarCargoWithdraw;
        //                     bot.is_tx = false;
        //                 };
        //             }
        //         }
        //     }
        // });

        Self {
            ctx,
            bots: bots_clone,
            tx,
            // _task,
        }
    }

    pub fn bots_run_update(&mut self, dt: std::time::Duration) -> anyhow::Result<()> {
        let _ = &self.bots.iter().for_each(|bot| {
            let bot = &mut bot.write().unwrap();
            bots::run_autoplay(bot, dt, &self.ctx, &self.tx).unwrap();
        });

        Ok(())
    }
}
