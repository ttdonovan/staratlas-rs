use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Program,
};

use staratlas_sage_sdk::{
    accounts::{Fleet, FleetState},
    derive,
};

use std::convert::TryFrom;
use std::rc::Rc;

use crate::{sage, traits};

pub struct Bot {
    pub fleet_id: Pubkey,
    pub fleet_acct: Fleet,
    pub fleet_state: FleetState,
    pub resource: Pubkey,
}

impl TryFrom<(&Program<Rc<Keypair>>, &Pubkey, &Pubkey)> for Bot {
    type Error = anyhow::Error;

    fn try_from(value: (&Program<Rc<Keypair>>, &Pubkey, &Pubkey)) -> Result<Self, Self::Error> {
        let (program, fleet_id, resource) = value;

        let (fleet, fleet_state) = derive::fleet_account_with_state(program, fleet_id)?;

        Ok(Bot {
            fleet_id: *fleet_id,
            fleet_acct: fleet,
            fleet_state,
            resource: *resource,
        })
    }
}

impl traits::FleetWithState for Bot {
    fn fleet_id(&self) -> &Pubkey {
        &self.fleet_id
    }

    fn fleet_acct(&self) -> &Fleet {
        &self.fleet_acct
    }

    fn fleet_state(&self) -> &FleetState {
        &self.fleet_state
    }
}

pub fn init(
    game_handler: &sage::GameHandler,
    fleet_id: &Pubkey,
    resource: &Pubkey,
) -> anyhow::Result<Bot> {
    Bot::try_from((&game_handler.sage_program, fleet_id, resource))
}
