use anchor_lang::prelude::Pubkey;
use anyhow::Result;

use std::str::FromStr;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use crate::{cli, cmds, sage_ctx};

#[derive(Debug)]
pub enum SageResponse {
    Fleet(sage_ctx::Fleet),
    FleetState(sage_ctx::FleetState),
    FleetWithState((sage_ctx::Fleet, sage_ctx::FleetState)),
    Planets(Vec<(Pubkey, sage_ctx::Planet)>),
}

pub struct SageHandler {
    tx: mpsc::Sender<cmds::Command>,
    rx: mpsc::Receiver<SageResponse>,
    _task: JoinHandle<Result<()>>,
}

impl SageHandler {
    pub fn send(&self, cmd: cmds::Command) -> Result<()> {
        self.tx.send(cmd)?;
        Ok(())
    }

    pub fn poll_response(&self) -> Option<SageResponse> {
        self.rx.try_recv().ok()
    }
}

pub fn init() -> SageHandler {
    use cmds::{Command, Inquiry};

    let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();
    let (resp_tx, resp_rx) = mpsc::channel::<SageResponse>();

    let _task = thread::spawn(move || -> Result<()> {
        let cli = cli::cli_parse();
        let client = cli::init_client(&cli)?;
        let (game_id, _) = cli::init_sage_config(&cli);
        let sage = sage_ctx::SageContext::new(&client, &game_id)?;

        while let Some(cmd) = cmd_rx.recv().ok() {
            let response = match cmd {
                Command::Find(find) => match find {
                    cmds::Find::Planets(sector) => {
                        let planets = sage.planet_accts(sector)?;
                        SageResponse::Planets(planets)
                    } // cmds::Find::Starbase(sector) => {
                      //     let _ sage.find_starbase_address(sector)?;
                      // }
                },
                Command::Inquiry(inquiry) => match inquiry {
                    Inquiry::Fleet(fleet_id) => {
                        let fleet_id = Pubkey::from_str(&fleet_id)?;
                        let (fleet, _) = sage.fleet_with_state_accts(&fleet_id)?;
                        SageResponse::Fleet(fleet)
                    }
                    Inquiry::FleetState(fleet_id) => {
                        let fleet_id = Pubkey::from_str(&fleet_id)?;
                        let (_, fleet_state) = sage.fleet_with_state_accts(&fleet_id)?;
                        SageResponse::FleetState(fleet_state)
                    }
                    Inquiry::FleetWithState(fleet_id) => {
                        let fleet_id = Pubkey::from_str(&fleet_id)?;
                        let (fleet, fleet_state) = sage.fleet_with_state_accts(&fleet_id)?;
                        SageResponse::FleetWithState((fleet, fleet_state))
                    }
                },
            };

            resp_tx.send(response)?;
        }

        Ok(())
    });

    SageHandler {
        tx: cmd_tx,
        rx: resp_rx,
        _task,
    }
}
