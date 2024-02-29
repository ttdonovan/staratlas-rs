use anchor_client::{
    solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signature, Signer},
    },
    Client, ClientError, Program,
};

use staratlas_sage_sdk::{
    derive, ixs,
    programs::{staratlas_cargo::ID as CARGO_ID, staratlas_sage::ID as SAGE_ID},
    Game,
};

use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;

use crate::traits;

fn sign_and_send<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    ixs: Vec<Instruction>,
) -> Result<Signature, ClientError> {
    let mut builder = program.request();

    for ix in ixs {
        builder = builder.instruction(ix);
    }

    builder.send()
}

pub struct GameHandler {
    pub sage_program: Program<Rc<Keypair>>,
    pub cargo_program: Program<Rc<Keypair>>,
    pub game_id: Pubkey,
    pub game_acct: Game,
}

impl TryFrom<(&Client<Rc<Keypair>>, &Pubkey)> for GameHandler {
    type Error = anyhow::Error;

    fn try_from(value: (&Client<Rc<Keypair>>, &Pubkey)) -> Result<Self, Self::Error> {
        let (client, game_id) = value;

        // Get the sage and cargo programs
        let sage_program = client.program(SAGE_ID)?;
        let cargo_program = client.program(CARGO_ID)?;

        // Sage game account
        let game = derive::game_account(&sage_program, &game_id)?;

        Ok(GameHandler {
            sage_program,
            cargo_program,
            game_id: *game_id,
            game_acct: game,
        })
    }
}

impl GameHandler {
    pub fn dock_to_starbase(
        &self,
        fleet: &impl traits::FleetWithState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::starbase::dock_to_starbase(
            &self.sage_program,
            (fleet.fleet_id(), (fleet.fleet_acct(), fleet.fleet_state())),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn undock_from_starbase(
        &self,
        fleet: &impl traits::FleetWithState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::starbase::undock_from_starbase(
            &self.sage_program,
            (fleet.fleet_id(), (fleet.fleet_acct(), fleet.fleet_state())),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn start_mining_asteroid(
        &self,
        fleet: &impl traits::FleetWithState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::mine::start_mining_asteroid(
            &self.sage_program,
            (fleet.fleet_id(), (fleet.fleet_acct(), fleet.fleet_state())),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn stop_mining_asteroid(
        &self,
        fleet: &impl traits::FleetWithState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::mine::stop_mining_asteroid(
            &self.sage_program,
            (fleet.fleet_id(), (fleet.fleet_acct(), fleet.fleet_state())),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn withdraw_from_fleet(
        &self,
        fleet: &impl traits::FleetWithState,
        starbase: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::cargo::withdraw_from_fleet(
            &self.sage_program,
            &self.cargo_program,
            (fleet.fleet_id(), fleet.fleet_acct()),
            (&self.game_id, &self.game_acct),
            starbase,
            mint,
            amount,
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn deposit_to_fleet(
        &self,
        fleet: &impl traits::FleetWithState,
        starbase: &Pubkey,
        cargo_pod_to: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::cargo::deposit_to_fleet(
            &self.sage_program,
            &self.cargo_program,
            (fleet.fleet_id(), fleet.fleet_acct()),
            (&self.game_id, &self.game_acct),
            starbase,
            cargo_pod_to,
            mint,
            amount,
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }
}

pub fn init_game_handler(
    client: &Client<Rc<Keypair>>,
    game_id: &Pubkey,
) -> anyhow::Result<GameHandler> {
    GameHandler::try_from((client, game_id))
}
