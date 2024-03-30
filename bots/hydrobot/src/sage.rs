use anchor_client::{
    solana_sdk::{
        compute_budget::ComputeBudgetInstruction,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signature, Signer},
    },
    Client, ClientError, Program,
};

use staratlas_sage_sdk::{
    accounts::Game,
    derive, ixs,
    programs::{staratlas_cargo::ID as CARGO_ID, staratlas_sage::ID as SAGE_ID},
};

use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;

use crate::traits;

fn sign_and_send<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    ixs: Vec<Vec<Instruction>>,
) -> Result<Signature, ClientError> {
    let mut signature = Signature::default();

    // `ixs` either [] (0 txs), [ix] (1 txs) or [ix, ix] (2 txs)
    for ix in ixs {
        let mut builder = program.request();

        // FIXME: this is a hack to set a the compute unit price for higher priority
        // Priority Fee added to each transaction in Lamports. Set to 0 (zero) to disable priority fees. 1 Lamport = 0.000000001 SOL
        let i = ComputeBudgetInstruction::set_compute_unit_price(5000);
        builder = builder.instruction(i);

        builder = ix
            .into_iter()
            .fold(builder, |builder, i| builder.instruction(i));

        // retry once on error
        signature = match builder.send() {
            Ok(signature) => signature,
            Err(_err) => builder.send()?,
        };
    }

    Ok(signature)
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
