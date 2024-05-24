use crate::{bots, cli, sage, Pubkey};

use std::sync::mpsc;
use std::thread::{self, JoinHandle};

mod autoplay;
mod txs;

#[derive(Debug, Clone)]
pub enum SageRequest {
    FleetWithState(Pubkey),
    AutoplayCargoTransport((Pubkey, sage::Fleet, sage::FleetState, bots::BotArgs)),
}

#[derive(Debug, Clone)]
pub enum SageResponse {
    FleetWithState((sage::Fleet, sage::FleetState)),
    UpdateFleetState(sage::FleetState),
    ExitWarp(sage::FleetState),
    WithdrawFromFleetCargoHold,
    NoOperation,
}

pub struct SageHandler {
    pub req_tx: mpsc::Sender<SageRequest>,
    _resp_tx: mpsc::Sender<SageResponse>,
    pub resp_rx: mpsc::Receiver<SageResponse>,
    _task: Option<JoinHandle<()>>,
}

impl SageHandler {
    pub fn new(game_id: Pubkey) -> Self {
        let (req_tx, req_rx) = mpsc::channel();
        let (resp_tx, resp_rx) = mpsc::channel();

        let _resp_tx = resp_tx.clone();
        let task = thread::spawn(move || {
            let cli = cli::cli_parse();
            let client = cli::init_client(&cli).unwrap();
            let sage = sage::SageContext::new(&client, &game_id).unwrap();

            while let Some(event) = req_rx.recv().ok() {
                log::info!("[Sage Request] - {:?}", event);

                let resp = match event {
                    SageRequest::FleetWithState(fleet_id) => {
                        let accounts = sage.fleet_with_state_accts(&fleet_id).unwrap();
                        SageResponse::FleetWithState(accounts)
                    }
                    SageRequest::AutoplayCargoTransport((fleet_id, fleet, state, bot_args)) => {
                        let (from_sector, to_sector, mint, _) = bot_args;

                        autoplay::cargo_transport(
                            &sage,
                            &fleet_id,
                            &fleet,
                            &state,
                            from_sector,
                            to_sector,
                            &mint,
                        )
                        .unwrap()
                    }
                };

                log::info!("[Sage Response] - {:?}", &resp);
                resp_tx.send(resp).unwrap();
            }
        });

        SageHandler {
            req_tx,
            _resp_tx,
            resp_rx,
            _task: Some(task),
        }
    }
}
