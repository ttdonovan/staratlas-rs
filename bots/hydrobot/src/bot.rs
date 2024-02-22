use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Program,
};

use staratlas_sage_sdk::{derive, Fleet, FleetState};

use std::convert::TryFrom;
use std::rc::Rc;

use crate::{sage, traits};

#[derive(Debug, PartialEq)]
pub enum Autoplay {
    Disabled,
    ManageHangarCargo,
    ReadyStarbaseDock,
    ReadyStarbaseUndock,
    StartMiningAsteroid,
    IsMiningAstroid,
}

pub struct Bot {
    pub fleet_id: Pubkey,
    pub fleet_acct: Fleet,
    pub fleet_state: FleetState,
    pub fleet_state_dirty: bool,
    pub resource: Pubkey,
    pub autoplay: bool,
    pub next_action: Autoplay,
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
            fleet_state_dirty: false,
            resource: *resource,
            autoplay: false,
            next_action: Autoplay::Disabled,
        })
    }
}

impl Bot {
    pub fn is_autoplay(&self, next_action: Autoplay) -> bool {
        self.autoplay && self.next_action == next_action
    }

    pub fn set_autoplay(&mut self, autoplay: bool) {
        self.autoplay = autoplay;
    }

    pub fn set_next_action(&mut self, next_action: Autoplay) {
        self.next_action = next_action;
    }

    pub fn set_fleet_sate(&mut self, fleet_state: FleetState) {
        self.fleet_state = fleet_state;
        self.fleet_state_dirty = false;
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
