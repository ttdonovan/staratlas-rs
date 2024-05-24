use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        compute_budget::ComputeBudgetInstruction,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{Keypair, Signature, Signer},
    },
    Client, ClientError, Program,
};
use spl_associated_token_account::get_associated_token_address;

use staratlas_sage_sdk::{
    derive, find, ixs,
    programs::{
        staratlas_cargo::ID as CARGO_ID,
        staratlas_sage::{state, ID as SAGE_ID},
    },
};

use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

pub use staratlas_sage_sdk::accounts::*;

// Priority Fee added to each transaction in Lamports. Set to 0 (zero) to disable priority fees. 1 Lamport = 0.000000001 SOL
const MICRO_LAMPORTS: u64 = 25_000; // 1_000_000

fn sign_and_send<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    ixs: Vec<Vec<Instruction>>,
) -> Result<Signature, ClientError> {
        let mut signature = Signature::default();

        // `ixs` either [] (0 txs), [ix] (1 txs) or [ix, ix] (2 txs)
        for ix in ixs {
            let mut builder = program.request();

            // FIXME: this is a hack to set a the compute unit price for higher priority
            // https://solana.com/developers/guides/advanced/how-to-request-optimal-compute
            let i = ComputeBudgetInstruction::set_compute_unit_price(MICRO_LAMPORTS);
            builder = builder.instruction(i);
            builder = ix.into_iter().fold(builder, |builder, i| builder.instruction(i));

            // signature = builder.send()?;

            // retry once on error
            signature = match builder.send() {
                Ok(signature) =>  { signature },
                Err(_err) => builder.send()?,
            };
        }

        Ok(signature)
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

    pub fn get_token_accounts_by_owner(&self, owner: &Pubkey) -> anyhow::Result<Vec<anchor_client::solana_client::rpc_response::RpcKeyedAccount>> {
        let accounts = self.rpc.get_token_accounts_by_owner(
            owner,
            anchor_client::solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::id()),
        )?;
        Ok(accounts)
    }

    pub fn get_token_account_balances_by_owner(&self, owner: &Pubkey) -> anyhow::Result<u32> {
        let accounts = self.get_token_accounts_by_owner(owner)?;

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
        Ok((*mine_item_id, mine_item.into()))
    }

    // pub fn starbase_address(&self, sector: [i64; 2]) -> anyhow::Result<Pubkey> {
    //     find::starbase_address(&self.game_id, sector)
    // }

    pub fn starbase_acct(&self, starbase_id: &Pubkey) -> anyhow::Result<(Pubkey, Starbase)> {
        let starbase = derive::starbase_account(&self.sage_program, starbase_id)?;
        Ok((*starbase_id, starbase))
    
    }

    pub fn starbase_cargo_pod_acct(&self, starbase_id: &Pubkey, fleet: &Fleet) -> anyhow::Result<(Pubkey, CargoPod)> {
        let player_profile = &fleet.owner_profile;
        let (_, starbase_acct) = self.starbase_acct(starbase_id)?;

        let (sage_player_profile, _) = find::sage_player_profile_address(&self.game_id, &player_profile);
        let (starbase_player, _) =
            find::starbase_player_address(&starbase_id, &sage_player_profile, starbase_acct.seq_id);

        let cargo_pods = derive::cargo_pod_accounts(&self.cargo_program, &starbase_player)?;
        let cargo_pod = cargo_pods
            .first()
            .expect("at least one cargo pod for starbase player");

        Ok(*cargo_pod)
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
            None,
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
        amount: Option<u64>,
    ) -> anyhow::Result<Option<Signature>> {
        let mut amount = match amount {
            Some(amount) => amount,
            None => {
                let address = get_associated_token_address(&fleet.cargo_hold, mint);
                get_balance(&self.rpc, &address) as u64
            },
        };

        amount -= 1; // leave 1 token behind

        if amount >= 1 {
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

impl SageContext {
    pub fn warp_to_coordinate(&self, fleet_id: &Pubkey, fleet: &Fleet, coordinate: [i64; 2]) -> anyhow::Result<Signature> {
        let ixs = ixs::warp::warp_to_coordinate(
            &self.sage_program,
            &self.cargo_program,
            (fleet_id, fleet),
            (&self.game_id, &self.game_acct),
            coordinate,
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)

    }

    pub fn ready_to_exit_warp(&self, fleet_id: &Pubkey, fleet: &Fleet) -> anyhow::Result<Signature> {
        let ixs = ixs::warp::ready_to_exit_warp(
            &self.sage_program,
            (fleet_id, fleet),
            (&self.game_id, &self.game_acct),
        )?;

        let signature = sign_and_send(&self.sage_program, ixs)?;

        Ok(signature)
    }
}