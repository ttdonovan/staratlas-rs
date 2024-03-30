use anchor_lang::prelude::Pubkey;

use std::str::FromStr;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use shared_sage_cli as cli;
use shared_sage_context as sage_ctx;

mod commands;
pub use commands::*;

pub fn deserialize(bytes: &[u8]) -> anyhow::Result<Reply> {
    let reply: Reply = borsh::from_slice(bytes)?;
    Ok(reply)
}

pub fn searlize(cmd: &Command) -> anyhow::Result<Vec<u8>> {
    let bytes = borsh::to_vec(&cmd)?;
    Ok(bytes)
}

pub fn sage_commander_sender_and_receiver() -> (
    mpsc::Sender<Command>,
    mpsc::Receiver<Reply>,
    JoinHandle<Result<(), anyhow::Error>>,
) {
    use commands::*;

    let (cmds_tx, cmds_rx) = mpsc::channel::<Command>();
    let (reply_tx, reply_rx) = mpsc::channel::<Reply>();

    let task = thread::spawn(move || -> Result<(), anyhow::Error> {
        let cli = cli::cli_parse();
        let client = cli::init_client(&cli)?;
        let (game_id, _) = cli::init_sage_config(&cli);
        let sage_ctx = sage_ctx::SageContext::new(&client, &game_id)?;

        while let Some(cmd) = cmds_rx.recv().ok() {
            let reply = match cmd {
                Command::Find(find) => match find {
                    Find::Planets(sector) => {
                        let planets = sage_ctx.planet_accts(sector)?;
                        Reply::Planets(planets)
                    } // cmds::Find::Starbase(sector) => {
                      //     let _ sage.find_starbase_address(sector)?;
                      // }
                },
                Command::Inquiry(inquiry) => match inquiry {
                    Inquiry::Fleet(fleet_id) => {
                        let fleet_id = Pubkey::from_str(&fleet_id)?;
                        let (fleet, _) = sage_ctx.fleet_with_state_accts(&fleet_id)?;
                        Reply::Fleet(fleet)
                    }
                    Inquiry::FleetState(fleet_id) => {
                        let fleet_id = Pubkey::from_str(&fleet_id)?;
                        let (_, fleet_state) = sage_ctx.fleet_with_state_accts(&fleet_id)?;
                        Reply::FleetState(fleet_state)
                    }
                    Inquiry::FleetWithState(fleet_id) => {
                        let fleet_id = Pubkey::from_str(&fleet_id)?;
                        let (fleet, fleet_state) = sage_ctx.fleet_with_state_accts(&fleet_id)?;
                        Reply::FleetWithState((fleet, fleet_state))
                    }
                },
            };

            reply_tx.send(reply)?;
        }

        Ok(())
    });

    (cmds_tx, reply_rx, task)
}
