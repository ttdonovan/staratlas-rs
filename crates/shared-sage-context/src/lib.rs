use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signature, Signer},
    },
    Client, ClientError, Program,
};
use spl_associated_token_account::get_associated_token_address;

use staratlas_sage_sdk::{
    derive, ixs,
    programs::{
        staratlas_cargo::ID as CARGO_ID,
        staratlas_sage::{state, ID as SAGE_ID},
    },
    Game, Planet,
};

use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

pub use staratlas_sage_sdk::{Fleet, FleetState, MineItem, Resource, Starbase};

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

fn get_balance(rpc: &RpcClient, address: &Pubkey) -> f64 {
    match rpc.get_token_account_balance(address) {
        Ok(balance) => balance.ui_amount.unwrap_or(0.0),
        Err(_) => 0.0,
    }
}

pub struct SageContext {
    pub sage_program: Program<Rc<Keypair>>,
    pub cargo_program: Program<Rc<Keypair>>,
    pub rpc: RpcClient,
    pub game_id: Pubkey,
    pub game_acct: Game,
}

impl SageContext {
    pub fn new(client: &Client<Rc<Keypair>>, game_id: &Pubkey) -> anyhow::Result<Self> {
        let sage_program = client.program(SAGE_ID)?;
        let cargo_program = client.program(CARGO_ID)?;

        let game = derive::game_account(&sage_program, &game_id)?;
        let rpc = sage_program.rpc();

        Ok(SageContext {
            sage_program,
            cargo_program,
            rpc,
            game_id: *game_id,
            game_acct: game,
        })
    }

    pub fn get_token_account_balances_by_owner(&self, owner: &Pubkey) -> anyhow::Result<u32> {
        let accounts = self.rpc.get_token_accounts_by_owner(
            owner,
            anchor_client::solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::id()),
        )?;

        // TODO: refactor and fix me
        let amount =
            accounts.iter().fold(0.0, |amount, keyed_acct| {
                let pubkey = Pubkey::from_str(&keyed_acct.pubkey).unwrap();
                let ui_amount = get_balance(&self.rpc, &pubkey);
                amount + ui_amount
            }) as u32;
    
        Ok(amount)
    }

    pub fn fleet_with_state_accts(&self, fleet_id: &Pubkey) -> anyhow::Result<(Fleet, FleetState)> {
        derive::fleet_account_with_state(&self.sage_program, fleet_id)
    }

    pub fn planet_accts(&self, sector: [i64; 2]) -> anyhow::Result<Vec<(Pubkey, Planet)>> {
        derive::planet_accounts(&self.sage_program, &self.game_id, sector)
    }

    pub fn resource_accts(&self, location: &Pubkey) -> anyhow::Result<Vec<(Pubkey, Resource)>> {
        derive::resource_accounts(&self.sage_program, &self.game_id, location)
    }

    pub fn mine_item_acct(&self, mine_item_id: &Pubkey) -> anyhow::Result<(Pubkey, MineItem)> {
        let mine_item =
            derive::derive_account::<_, state::MineItem>(&self.sage_program, mine_item_id)?;
        Ok((*mine_item_id, MineItem(mine_item)))
    }

    pub fn starbase_acct(&self, starbase_id: &Pubkey) -> anyhow::Result<(Pubkey, Starbase)> {
        // let starbase =
        //     derive::derive_account::<_, state::Starbase>(&self.sage_program, starbase_id)?;
        let starbase = derive::starbase_account(&self.sage_program, starbase_id)?;
        Ok((*starbase_id, starbase))
    
    }
}

impl SageContext {
    pub fn start_mining_asteroid(
        &self,
        fleet_id: &Pubkey,
        fleet: &Fleet,
        state: &FleetState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::mine::start_mining_asteroid(
            &self.sage_program,
            (fleet_id, (fleet, state)),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn stop_mining_asteroid(
        &self,
        fleet_id: &Pubkey,
        fleet: &Fleet,
        state: &FleetState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::mine::stop_mining_asteroid(
            &self.sage_program,
            (fleet_id, (fleet, state)),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }
}

impl SageContext {
    pub fn dock_to_starbase(
        &self,
        fleet_id: &Pubkey,
        fleet: &Fleet,
        state: &FleetState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::starbase::dock_to_starbase(
            &self.sage_program,
            (fleet_id, (fleet, state)),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn undock_from_starbase(
        &self,
        fleet_id: &Pubkey,
        fleet: &Fleet,
        state: &FleetState,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::starbase::undock_from_starbase(
            &self.sage_program,
            (fleet_id, (fleet, state)),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }
}

impl SageContext {
    pub fn deposit_to_fleet(
        &self,
        fleet_id: &Pubkey,
        fleet: &Fleet,
        starbase: &Pubkey,
        cargo_pod_to: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<Signature> {
        let ixs = ixs::cargo::deposit_to_fleet(
            &self.sage_program,
            &self.cargo_program,
            (fleet_id, fleet),
            (&self.game_id, &self.game_acct),
            starbase,
            cargo_pod_to,
            mint,
            amount,
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }

    pub fn withdraw_from_fleet(
        &self,
        fleet_id: &Pubkey,
        fleet: &Fleet,
        starbase: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<Option<Signature>> {
        let mut amount = if amount == u64::MAX {
            let address = get_associated_token_address(&fleet.0.cargo_hold, mint);
            get_balance(&self.rpc, &address) as u64
        } else {
            amount
        };

        amount -= 1; // leave 1 token behind

        if amount > 0 {
            let ixs = ixs::cargo::withdraw_from_fleet(
                &self.sage_program,
                &self.cargo_program,
                (fleet_id, fleet),
                (&self.game_id, &self.game_acct),
                starbase,
                mint,
                amount,
            )?;
    
            let signature = sign_and_send(&self.sage_program, ixs)?;
    
            Ok(Some(signature))
        } else {
            Ok(None)
        }
    }
}