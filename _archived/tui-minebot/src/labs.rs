use anchor_client::{anchor_lang::prelude::Pubkey, solana_sdk::signature::Signature};

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use crate::{bots, cli, sage};

mod txs;

pub enum SageRequest {
    RefreshFleet(Pubkey),
    StartMiningAsteroid((Pubkey, sage::Fleet, sage::FleetState)),
    StopMiningAsteroid((Pubkey, sage::Fleet, sage::FleetState)),
    DockToStarbase((Pubkey, sage::Fleet, sage::FleetState)),
    UndockFromStarbase((Pubkey, sage::Fleet, sage::FleetState)),
    StarbaseHangarCargoWithdraw((Pubkey, sage::Fleet, Pubkey, Pubkey)), // (fleet_id, fleet, starbase_id, mint)
    StarbaseHangarDepositToFleet(Pubkey, sage::Fleet, Pubkey, Pubkey, bots::CargoDeposit, u64), // (fleet_id, fleet, starbase_id, cargo_pod_to, cargo_deposit, amount)
}

pub enum SageResponse {
    RefreshFleet(
        Pubkey,
        (
            sage::Fleet,
            sage::FleetState,
            (u32, u32, u32),
            Option<(Pubkey, sage::Resource)>,
            Option<(Pubkey, sage::MineItem)>,
        ),
    ), // (fuel, ammo, cargo))
    StartMiningAsteroid((Pubkey, Result<(Signature, sage::FleetState), anyhow::Error>)),
    StopMiningAsteroid((Pubkey, Result<(Signature, sage::FleetState), anyhow::Error>)),
    DockToStarbase(
        (
            Pubkey,
            Result<(Signature, sage::FleetState, (u32, u32, u32)), anyhow::Error>,
        ),
    ), // (fuel, ammo, cargo)
    UndockFromStarbase(
        (
            Pubkey,
            Result<(Signature, sage::FleetState, (u32, u32, u32)), anyhow::Error>,
        ),
    ), // (fuel, ammo, cargo)
    StarbaseHangarCargoWithdraw((Pubkey, Result<Option<Signature>, anyhow::Error>)),
    StarbaseHangarDepositToFleet(
        (
            (Pubkey, bots::CargoDeposit), // (fleet_id, cargo_deposit)
            Result<Signature, anyhow::Error>,
        ),
    ),
}

pub struct SageLabsHandler {
    pub bots: HashMap<Pubkey, bots::MiningBot>,
    pub game_id: Pubkey,
    fleet_ids: Vec<Pubkey>,
    tx: mpsc::Sender<SageRequest>,
    rx: mpsc::Receiver<SageResponse>,
    _task: JoinHandle<Result<(), anyhow::Error>>,
}

impl SageLabsHandler {
    pub fn new(game_id: Pubkey, fleet_ids: Vec<Pubkey>) -> Self {
        let (tx, rx) = mpsc::channel::<SageRequest>();
        let (tx_2, rx_2) = mpsc::channel::<SageResponse>();

        let _task = thread::spawn(move || -> Result<(), anyhow::Error> {
            // this 'task' thread needs it's own client and sage context
            let cli = cli::cli_parse();
            let client = cli::init_client(&cli)?;
            let (game_id, _) = cli::init_sage_config(&cli);
            let sage = sage::SageContext::new(&client, &game_id)?;

            while let Some(request) = rx.recv().ok() {
                let response = match request {
                    SageRequest::RefreshFleet(fleet_id) => {
                        txs::refresh_fleet_response(&sage, &fleet_id)?
                    }
                    SageRequest::StartMiningAsteroid((fleet_id, fleet, state)) => {
                        txs::start_mining_asteroid_response(&sage, &fleet_id, &fleet, &state)?
                    }
                    SageRequest::StopMiningAsteroid((fleet_id, fleet, state)) => {
                        txs::stop_mining_asteroid_response(&sage, &fleet_id, &fleet, &state)?
                    }
                    SageRequest::DockToStarbase((fleet_id, fleet, state)) => {
                        txs::dock_to_starbase_response(&sage, &fleet_id, &fleet, &state)?
                    }
                    SageRequest::UndockFromStarbase((fleet_id, fleet, state)) => {
                        txs::undock_from_starbase_response(&sage, &fleet_id, &fleet, &state)?
                    }
                    SageRequest::StarbaseHangarCargoWithdraw((
                        fleet_id,
                        fleet,
                        starbase_id,
                        mint,
                    )) => txs::starbase_hangar_cargo_withdraw_response(
                        &sage,
                        &fleet_id,
                        &fleet,
                        &starbase_id,
                        &mint,
                    )?,
                    SageRequest::StarbaseHangarDepositToFleet(
                        fleet_id,
                        fleet,
                        starbase_id,
                        cargo_pod_to,
                        cargo_deposit,
                        amount,
                    ) => txs::starbase_hangar_cargo_deposit_to_fleet_response(
                        &sage,
                        &fleet_id,
                        &fleet,
                        &starbase_id,
                        &cargo_pod_to,
                        cargo_deposit,
                        amount,
                    )?,
                };

                tx_2.send(response)?;
            }

            Ok(())
        });

        Self {
            bots: HashMap::new(),
            game_id,
            fleet_ids,
            tx,
            rx: rx_2,
            _task,
        }
    }

    pub fn refresh_fleet(&self) -> anyhow::Result<()> {
        for fleet_id in self.fleet_ids.iter() {
            self.tx.send(SageRequest::RefreshFleet(*fleet_id))?;
        }

        Ok(())
    }

    pub fn run_bots(&mut self, dt: std::time::Duration) -> anyhow::Result<()> {
        for (_, mut bot) in self.bots.iter_mut() {
            bots::run::autoplay(&mut bot, dt, &self.tx)?;
        }

        Ok(())
    }

    pub fn poll_response(&mut self) -> anyhow::Result<()> {
        if let Some(response) = self.rx.try_recv().ok() {
            match response {
                SageResponse::RefreshFleet(fleet_id, data) => {
                    let bot = bots::process::refresh_fleet(fleet_id, data)?;
                    self.bots.insert(fleet_id, bot);
                }
                SageResponse::StartMiningAsteroid((fleet_id, result)) => {
                    if let Some(bot) = self.bots.get_mut(&fleet_id) {
                        bots::process::start_mining_asteroid_result(bot, result);
                    };
                }
                SageResponse::StopMiningAsteroid((fleet_id, result)) => {
                    if let Some(bot) = self.bots.get_mut(&fleet_id) {
                        bots::process::stop_mining_asteroid_result(bot, result);
                    };
                }
                SageResponse::DockToStarbase((fleet_id, result)) => {
                    if let Some(bot) = self.bots.get_mut(&fleet_id) {
                        bots::process::dock_to_starbase_result(bot, result);
                    };
                }
                SageResponse::UndockFromStarbase((fleet_id, result)) => {
                    if let Some(bot) = self.bots.get_mut(&fleet_id) {
                        bots::process::undock_from_starbase_result(bot, result);
                    };
                }
                SageResponse::StarbaseHangarCargoWithdraw((fleet_id, result)) => {
                    if let Some(bot) = self.bots.get_mut(&fleet_id) {
                        bots::process::starbase_hangar_cargo_withdraw_result(bot, result);
                    };
                }
                SageResponse::StarbaseHangarDepositToFleet(((fleet_id, cargo_deposit), result)) => {
                    if let Some(bot) = self.bots.get_mut(&fleet_id) {
                        bots::process::starbase_hangar_deposit_to_fleet_result(
                            bot,
                            cargo_deposit,
                            result,
                        );
                    };
                }
            }
        }

        Ok(())
    }
}
