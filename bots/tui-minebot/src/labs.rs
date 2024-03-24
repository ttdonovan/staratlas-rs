use anchor_client::solana_sdk::signature::Signature;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};

use crate::bots::CargoDeposit;
use crate::{bots, cli, sage};

pub enum Event {
    StartMiningAsteroid(usize),
    StopMiningAsteroid(usize),
    DockToStarbase(usize),
    UndockFromStarbase(usize),
    StarbaseHangarCargoWithdraw(usize),
    StarbaseHangarDepositToFleet(usize, bots::CargoDeposit),
}

pub enum Response {
    StartMiningAsteroid((usize, Result<Signature, anyhow::Error>)),
    StopMiningAsteroid((usize, Result<Signature, anyhow::Error>)),
    DockToStarbase((usize, Result<Signature, anyhow::Error>)),
    UndockFromStarbase((usize, Result<Signature, anyhow::Error>)),
    StarbaseHangarCargoWithdraw((usize, Result<Option<Signature>, anyhow::Error>)),
    StarbaseHangarDepositToFleet(
        (
            (usize, bots::CargoDeposit),
            Result<Signature, anyhow::Error>,
        ),
    ),
}

pub struct SageLabs {
    pub ctx: sage::SageContext,
    pub bots: Arc<Vec<RwLock<bots::MiningBot>>>,
    tx: mpsc::Sender<Event>,
    rx: mpsc::Receiver<Response>,
    _task: JoinHandle<()>,
}

impl SageLabs {
    pub fn new(ctx: sage::SageContext, bots: Vec<bots::MiningBot>) -> Self {
        let (tx, rx) = mpsc::channel::<Event>();
        let (tx_2, rx_2) = mpsc::channel::<Response>();

        let bots: Arc<Vec<RwLock<bots::MiningBot>>> =
            Arc::new(bots.into_iter().map(RwLock::new).collect());
        let bots_clone = Arc::clone(&bots);

        let _task = thread::spawn(move || {
            // this 'task' thread needs it's own client and sage context
            let cli = cli::cli_parse();
            let client = cli::init_client(&cli).unwrap();
            let (game_id, _) = cli::init_sage_config(&cli);
            let sage = sage::SageContext::new(&client, &game_id).unwrap();

            let bots = bots.as_ref();
            while let Some(event) = rx.recv().ok() {
                match event {
                    Event::StartMiningAsteroid(index) => {
                        let bot = bots[index].read().unwrap();

                        log::info!(
                            "SageLabs > Start Mining Asteroid ({}): {}",
                            bot.masked_fleet_id(),
                            &ctx.game_id
                        );

                        let (fleet_id, _, _, fleet, state) = &bot.fleet;
                        let res = sage.start_mining_asteroid(fleet_id, fleet, state);

                        tx_2.send(Response::StartMiningAsteroid((index, res)))
                            .unwrap();
                    }
                    Event::StopMiningAsteroid(index) => {
                        let bot = bots[index].read().unwrap();

                        log::info!(
                            "SageLabs > Stop Mining Asteroid ({}): {}",
                            bot.masked_fleet_id(),
                            &ctx.game_id
                        );

                        let (fleet_id, _, _, fleet, state) = &bot.fleet;
                        let res = sage.stop_mining_asteroid(fleet_id, fleet, state);

                        tx_2.send(Response::StopMiningAsteroid((index, res)))
                            .unwrap();
                    }
                    Event::DockToStarbase(index) => {
                        let bot = bots[index].read().unwrap();

                        log::info!(
                            "SageLabs > Dock to Starbase ({}): {}",
                            bot.masked_fleet_id(),
                            &ctx.game_id
                        );

                        let (fleet_id, _, _, fleet, state) = &bot.fleet;
                        let res = sage.dock_to_starbase(fleet_id, fleet, state);

                        tx_2.send(Response::DockToStarbase((index, res))).unwrap();
                    }
                    Event::UndockFromStarbase(index) => {
                        let bot = bots[index].write().unwrap();

                        log::info!(
                            "SageLabs > Undock from Starbase ({}): {}",
                            bot.masked_fleet_id(),
                            &ctx.game_id
                        );

                        let (fleet_id, _, _, fleet, state) = &bot.fleet;
                        let res = sage.undock_from_starbase(fleet_id, fleet, state);

                        tx_2.send(Response::UndockFromStarbase((index, res)))
                            .unwrap();
                    }
                    Event::StarbaseHangarCargoWithdraw(index) => {
                        let bot = bots[index].read().unwrap();

                        log::info!(
                            "SageLabs > Starbase Hangar Cargo Withdraw ({}): {}",
                            bot.masked_fleet_id(),
                            &ctx.game_id
                        );

                        let (fleet_id, _, _, fleet, _) = &bot.fleet;
                        let starbase = bot.starbase_id().unwrap();
                        let mine_item = &bot.mine_item.2;
                        let mint = &mine_item.0.mint;

                        let res = sage.withdraw_from_fleet(
                            fleet_id, fleet, starbase, mint, None, // withdraw all (max)
                        );

                        tx_2.send(Response::StarbaseHangarCargoWithdraw((index, res)))
                            .unwrap();
                    }
                    Event::StarbaseHangarDepositToFleet(index, deposit) => {
                        let bot = bots[index].read().unwrap();

                        log::info!(
                            "SageLabs > Starbase Hangar Deposit to Fleet ({}): {:?}",
                            bot.masked_fleet_id(),
                            deposit
                        );

                        let (cargo_pod_to, mint, amount) = match deposit {
                            bots::CargoDeposit::Fuel => {
                                let (fuel_tank, actual, capacity) = bot.fuel_tank.clone();
                                let fuel_mint = &sage.game_acct.0.mints.fuel;
                                let amount = (capacity - actual) as u64;

                                (fuel_tank, fuel_mint, amount)
                            }
                            bots::CargoDeposit::Ammo => {
                                let (ammo_bank, actual, capacity) = bot.ammo_bank.clone();
                                let ammo_mint = &sage.game_acct.0.mints.ammo;
                                let amount = (capacity - actual) as u64;

                                (ammo_bank, ammo_mint, amount)
                            }
                            bots::CargoDeposit::Food => {
                                let (cargo_hold, _actual, capacity) = bot.cargo_hold.clone();
                                let food_mint = &sage.game_acct.0.mints.food;
                                let amount = (capacity as f32 * 0.05) as u64;

                                (cargo_hold, food_mint, amount)
                            }
                        };

                        let (fleet_id, _, _, fleet, _) = &bot.fleet;
                        let starbase = bot.starbase_id().unwrap();

                        let res = sage.deposit_to_fleet(
                            fleet_id,
                            fleet,
                            starbase,
                            &cargo_pod_to,
                            mint,
                            amount,
                        );

                        tx_2.send(Response::StarbaseHangarDepositToFleet((
                            (index, deposit),
                            res,
                        )))
                        .unwrap();
                    }
                }
            }
        });

        Self {
            ctx,
            bots: bots_clone,
            tx,
            rx: rx_2,
            _task,
        }
    }

    pub fn run_bots(&mut self, dt: std::time::Duration) -> anyhow::Result<()> {
        for bot in self.bots.iter().enumerate() {
            let (index, bot) = bot;
            let mut bot = bot.write().unwrap();
            bots::run_autoplay(&mut bot, index, dt, &self.ctx, &self.tx).unwrap();
        }

        Ok(())
    }

    pub fn poll_response(&self) {
        if let Some(response) = self
            .rx
            .recv_timeout(std::time::Duration::from_millis(200))
            .ok()
        {
            let bots = self.bots.as_ref();

            match response {
                Response::StartMiningAsteroid((index, res)) => {
                    let mut bot = bots[index].write().unwrap();
                    match res {
                        Ok(signature) => {
                            bot.autoplay = bots::Autoplay::IsMiningAsteroid;
                            bot.is_fleet_state_dirty = true;
                            bot.txs = Some(signature);
                            bot.txs_counter += 1;
                            bot.is_tx = false;

                            // start our mining timer
                            bot.reset_mining_timer();
                        }
                        Err(err) => {
                            log::error!(
                                "[{}] Start Mining Asteroid: {:?}",
                                bot.masked_fleet_id(),
                                err
                            );
                            bot.txs_errors += 1;
                            bot.is_tx = false;
                        }
                    }
                }
                Response::StopMiningAsteroid((index, res)) => {
                    let mut bot = bots[index].write().unwrap();
                    match res {
                        Ok(signature) => {
                            bot.autoplay = bots::Autoplay::StarbaseDock;
                            bot.is_fleet_state_dirty = true;
                            bot.txs = Some(signature);
                            bot.txs_counter += 1;
                            bot.is_tx = false;
                        }
                        Err(err) => {
                            log::error!(
                                "[{}] Stop Mining Asteroid: {:?}",
                                bot.masked_fleet_id(),
                                err
                            );
                            bot.txs_errors += 1;
                            bot.is_tx = false;
                        }
                    }
                }
                Response::DockToStarbase((index, res)) => {
                    let mut bot = bots[index].write().unwrap();
                    match res {
                        Ok(signature) => {
                            bot.autoplay = bots::Autoplay::StarbaseHangarCargoWithdraw;
                            bot.is_fleet_state_dirty = true;
                            bot.txs = Some(signature);
                            bot.txs_counter += 1;
                            bot.is_tx = false;
                        }
                        Err(err) => {
                            log::error!("[{}] Dock to Starbase: {:?}", bot.masked_fleet_id(), err);
                            bot.txs_errors += 1;
                            bot.is_tx = false;
                        }
                    }
                }
                Response::UndockFromStarbase((index, res)) => {
                    let mut bot = bots[index].write().unwrap();
                    match res {
                        Ok(signature) => {
                            bot.autoplay = bots::Autoplay::IsIdle;
                            bot.is_fleet_state_dirty = true;
                            bot.txs = Some(signature);
                            bot.txs_counter += 1;
                            bot.is_tx = false;
                        }
                        Err(err) => {
                            log::error!(
                                "[{}] Undock from Starbase: {:?}",
                                bot.masked_fleet_id(),
                                err
                            );
                            bot.txs_errors += 1;
                            bot.is_tx = false;
                        }
                    }
                }
                Response::StarbaseHangarCargoWithdraw((index, res)) => {
                    let mut bot = bots[index].write().unwrap();
                    match res {
                        Ok(signature) => {
                            bot.autoplay = bots::Autoplay::StarbaseHangarCargoDeposit(
                                bots::CargoDeposit::Fuel,
                            );
                            bot.is_fleet_state_dirty = true;
                            bot.txs = signature;
                            bot.txs_counter += 1;
                            bot.is_tx = false;
                        }
                        Err(err) => {
                            log::error!(
                                "[{}] Widthdraw from Fleet: {:?}",
                                bot.masked_fleet_id(),
                                err
                            );
                            bot.txs_errors += 1;
                            bot.is_tx = false;
                        }
                    }
                }
                Response::StarbaseHangarDepositToFleet((value, res)) => {
                    let (index, deposit) = value;
                    let mut bot = bots[index].write().unwrap();
                    match res {
                        Ok(signature) => {
                            bot.is_fleet_state_dirty = true;
                            bot.txs = Some(signature);
                            bot.txs_counter += 1;
                            bot.is_tx = false;

                            bot.autoplay = match deposit {
                                CargoDeposit::Fuel => {
                                    bots::Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Ammo)
                                }
                                CargoDeposit::Ammo => {
                                    bots::Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Food)
                                }
                                CargoDeposit::Food => bots::Autoplay::StarbaseUndock,
                            };
                        }
                        Err(err) => {
                            log::error!("[{}] Deposit to Fleet: {:?}", bot.masked_fleet_id(), err);
                            bot.txs_errors += 1;
                            bot.is_tx = false;
                        }
                    }
                }
            }
        }
    }
}
