use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::{
        nonblocking::rpc_client::RpcClient,
        rpc_request::TokenAccountsFilter,
        rpc_response::{Response, RpcSimulateTransactionResult},
    },
    solana_sdk::{
        compute_budget::ComputeBudgetInstruction,
        instruction::Instruction,
        signature::{Keypair, Signature, Signer},
        transaction::Transaction,
    },
    ClientError, Program,
};
use borsh::BorshDeserialize;
use solana_account_decoder::UiAccountData;

use std::ops::Deref;

mod accounts;
pub mod addr;
pub mod calc;
pub mod derive;
pub mod ixs;
pub mod program;

pub use accounts::*;

const MICRO_LAMPORTS: u64 = 100;

pub struct SageBasedGameHandler {}

// Fleet (and State)
impl SageBasedGameHandler {
    pub async fn get_fleet_with_state<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        fleet_id: &Pubkey,
    ) -> Result<FleetWithState, ClientError> {
        let rpc = program.async_rpc();
        let account = rpc.get_account(&fleet_id).await?;
        let mut account_data = account.data.as_slice();

        let fleet_with_state = FleetWithState::deserialize(&mut account_data)?;
        Ok(fleet_with_state)
    }
}

// Starbase (Dock and Undock)
impl SageBasedGameHandler {
    pub async fn dock_to_starbase<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        payer: &Keypair,
        game: (&Pubkey, &Game),
        fleet: (&Pubkey, &Fleet),
        sector: [i64; 2],
    ) -> Option<Result<Signature, ClientError>> {
        let ix = ixs::dock_to_starbase(&program, game, fleet, sector);
        Self::simulate_and_send_transaction(program, payer, &vec![ix]).await
    }

    pub async fn undock_from_starbase<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        payer: &Keypair,
        game: (&Pubkey, &Game),
        fleet: (&Pubkey, &Fleet),
        starbase: &Pubkey,
    ) -> Option<Result<Signature, ClientError>> {
        let ix = ixs::undock_from_starbase(&program, game, fleet, starbase);
        Self::simulate_and_send_transaction(program, payer, &vec![ix]).await
    }
}

// Cargo (Deposit and Withdraw)
impl SageBasedGameHandler {
    pub async fn cargo_deposit_to_fleet<C: Deref<Target = impl Signer> + Clone>(
        sage_program: &Program<C>,
        cargo_program: &Program<C>,
        payer: &Keypair,
        game: (&Pubkey, &Game),
        fleet: (&Pubkey, &Fleet),
        starbase: &Pubkey,
        cargo_pod_to: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Option<Result<Signature, ClientError>> {
        let (game_id, _) = game;
        let (_, fleet_act) = fleet;
        let player_profile = &fleet_act.owner_profile;

        let starbase_seq_id = 0;
        let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);
        let (starbase_player, _) =
            addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

        let cargo_pods = derive::cargo_pod_accounts(cargo_program, &starbase_player)
            .await
            .unwrap();
        let (cargo_pod_id, cargo_pod) = cargo_pods[0];

        let ix: Instruction = ixs::cargo_deposit_to_fleet(
            sage_program,
            game,
            fleet,
            (&cargo_pod_id, &cargo_pod),
            starbase,
            cargo_pod_to,
            mint,
            amount,
        );

        Self::simulate_and_send_transaction(sage_program, payer, &vec![ix]).await
    }

    pub async fn cargo_withdraw_from_fleet<C: Deref<Target = impl Signer> + Clone>(
        sage_program: &Program<C>,
        cargo_program: &Program<C>,
        payer: &Keypair,
        game: (&Pubkey, &Game),
        fleet: (&Pubkey, &Fleet),
        starbase: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Option<Result<Signature, ClientError>> {
        let (game_id, _) = game;
        let (_, fleet_act) = fleet;
        let player_profile = &fleet_act.owner_profile;

        let starbase_seq_id = 0;
        let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);
        let (starbase_player, _) =
            addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

        let cargo_pods = derive::cargo_pod_accounts(cargo_program, &starbase_player)
            .await
            .unwrap();
        let (cargo_pod_id, cargo_pod) = cargo_pods[0];

        let ix: Instruction = ixs::cargo_withdraw_from_fleet(
            sage_program,
            game,
            fleet,
            (&cargo_pod_id, &cargo_pod),
            starbase,
            mint,
            amount,
        );
        SageBasedGameHandler::simulate_and_send_transaction(sage_program, payer, &vec![ix]).await
    }
}

// Asteroid Mining (Start and Stop)
impl SageBasedGameHandler {
    pub async fn start_mining<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        payer: &Keypair,
        game: (&Pubkey, &Game),
        fleet: (&Pubkey, &Fleet),
        mine_item: &Pubkey,
        resource: &Pubkey,
        planet: &Pubkey,
        sector: [i64; 2],
    ) -> Option<Result<Signature, ClientError>> {
        let ix =
            ixs::start_mining_asteroid(&program, game, fleet, mine_item, resource, planet, sector);
        Self::simulate_and_send_transaction(program, payer, &vec![ix]).await
    }

    pub async fn stop_mining<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        payer: &Keypair,
        game: (&Pubkey, &Game),
        fleet: (&Pubkey, &Fleet),
        mine_item: &Pubkey,
        mine_item_mint: &Pubkey,
        resource: &Pubkey,
        planet: &Pubkey,
        sector: [i64; 2],
    ) -> Option<Result<Signature, ClientError>> {
        let mut ixs = vec![];

        let ix = ixs::fleet_state_handler(
            &program,
            game,
            fleet,
            mine_item,
            mine_item_mint,
            resource,
            planet,
            sector,
        );
        ixs.push(ix);

        let ix = ixs::stop_mining_asteroid(&program, game, fleet, mine_item, resource, planet);
        ixs.push(ix);

        let mut last_signature: Option<Result<Signature, ClientError>> = None;

        for ix in ixs {
            match Self::simulate_and_send_transaction(program, payer, &vec![ix]).await {
                Some(Ok(signature)) => {
                    last_signature = Some(Ok(signature));
                }
                Some(Err(e)) => {
                    last_signature = Some(Err(e));
                    break;
                }
                _ => {
                    return None;
                }
            }
        }

        return last_signature;
    }
}

// Token Accounts (Parsed)
impl SageBasedGameHandler {
    pub async fn parsed_token_account_amounts(
        rpc: &RpcClient,
        owner: &Pubkey,
    ) -> Vec<(String, u64)> {
        let accounts = rpc
            .get_token_accounts_by_owner(owner, TokenAccountsFilter::ProgramId(spl_token::id()))
            .await
            .unwrap();

        let token_amounts: Vec<(String, u64)> = accounts
            .iter()
            .filter_map(|account| {
                let parsed_account = match &account.account.data {
                    UiAccountData::Json(parsed_account) => parsed_account,
                    _ => return None,
                };

                let parsed = &parsed_account.parsed;
                let info = match parsed.get("info").and_then(|v| v.as_object()) {
                    Some(info) => info,
                    None => return None,
                };

                let mint = info
                    .get("mint")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let amount = info
                    .get("tokenAmount")
                    .and_then(|v| v.as_object())
                    .and_then(|v| v.get("amount"))
                    .and_then(|v| v.as_str())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0);

                Some((mint, amount))
            })
            .collect();

        token_amounts
    }
}

// Simulate and Send Transaction
impl SageBasedGameHandler {
    pub async fn simulate_transaction<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        instructions: &Vec<Instruction>,
        signers: &Vec<&dyn Signer>,
    ) -> Result<Response<RpcSimulateTransactionResult>, ClientError> {
        let rpc_client = program.async_rpc();
        let recent_blockhash = rpc_client.get_latest_blockhash().await?;

        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&program.payer()),
            signers,
            recent_blockhash,
        );

        let response = rpc_client.simulate_transaction(&tx).await?;
        Ok(response)
    }

    pub async fn send_transaction<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        instructions: &Vec<Instruction>,
        _units_consumed: Option<u64>,
    ) -> Result<Signature, ClientError> {
        // protection against sending transactions to a program that is not the Sage program
        assert_eq!(program.id(), program::SAGE_ID, "invalid program id");

        let mut builder = program.request();

        let i = ComputeBudgetInstruction::set_compute_unit_price(MICRO_LAMPORTS);
        builder = builder.instruction(i);

        // if let Some(units) = units_consumed {
        //     // TODO: add some margin as buffer 20/25% to units
        //     let i = ComputeBudgetInstruction::set_compute_unit_limit(units as u32);
        //     builder = builder.instruction(i);
        // }

        builder = instructions
            .into_iter()
            .fold(builder, |builder, i| builder.instruction(i.clone()));

        builder.send().await
    }

    pub async fn simulate_and_send_transaction<C: Deref<Target = impl Signer> + Clone>(
        program: &Program<C>,
        payer: &Keypair,
        instructions: &Vec<Instruction>,
    ) -> Option<Result<Signature, ClientError>> {
        match Self::simulate_transaction(program, instructions, &vec![payer]).await {
            Ok(simulation) => {
                log::info!("{:#?}", simulation.value.logs);
                log::info!("Units Consumed {:?}", simulation.value.units_consumed);

                if simulation.value.err.is_none() {
                    let result = Self::send_transaction(
                        program,
                        instructions,
                        simulation.value.units_consumed,
                    )
                    .await;
                    return Some(result);
                } else {
                    log::error!("{:?}", simulation.value.err);
                    log::error!("{:?}", instructions);

                    return None;
                }
            }
            Err(e) => {
                log::error!("{:?}", &e);
                return Some(Err(e));
            }
        }
    }
}
