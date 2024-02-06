use anchor_client::{
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, signature::Signer},
    Program,
};
use anchor_lang::{prelude::Pubkey, AnchorDeserialize, Id};
use solana_account_decoder::UiAccountEncoding;

use staratlas_sage::{program::Sage, state, typedefs};

use crate::{utils::str_to_u8_32, Fleet, FleetState};

use std::ops::Deref;

pub fn derive_fleet_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_pubkey: &Pubkey,
    player_profile_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Fleet)>> {
    let accounts = program.accounts::<state::Fleet>(vec![
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(9, game_pubkey.as_ref())),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            41,
            player_profile_pubkey.as_ref(),
        )),
    ])?;

    let fleet_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, Fleet(account.clone())))
        .collect();

    Ok(fleet_accounts)
}

pub fn derive_fleet_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    fleet_pubkey: &Pubkey,
) -> anyhow::Result<Fleet> {
    let account = program.account::<state::Fleet>(*fleet_pubkey)?;
    Ok(Fleet(account))
}

pub fn derive_fleet_account_with_state<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    fleet_pubkey: &Pubkey,
) -> anyhow::Result<(Fleet, FleetState)> {
    let account = get_fleet_account(program, fleet_pubkey)?;
    let account_data = account.data.as_slice();

    // let _ = account_data[..8]; // what are these 8 bytes?

    let mut account_data = &account_data[8..];
    let fleet = state::Fleet::deserialize(&mut account_data)?;

    let remaining_data = account_data;
    let discriminator = remaining_data[0];
    let mut remaining_data = &remaining_data[1..];

    let fleet_state = match discriminator {
        0 => {
            let starbase_loading_bay =
                typedefs::StarbaseLoadingBay::deserialize(&mut remaining_data)?;
            FleetState::StarbaseLoadingBay(starbase_loading_bay)
        }
        1 => {
            let idle = typedefs::Idle::deserialize(&mut remaining_data)?;
            FleetState::Idle(idle)
        }
        2 => {
            let mine_astriod = typedefs::MineAsteroid::deserialize(&mut remaining_data)?;
            FleetState::MineAsteroid(mine_astriod)
        }
        3 => {
            let move_warp = typedefs::MoveWarp::deserialize(&mut remaining_data)?;
            FleetState::MoveWarp(move_warp)
        }
        4 => {
            let move_subwarp = typedefs::MoveSubwarp::deserialize(&mut remaining_data)?;
            FleetState::MoveSubwarp(move_subwarp)
        }
        5 => {
            let respawn = typedefs::Respawn::deserialize(&mut remaining_data)?;
            FleetState::Respawn(respawn)
        }
        _ => {
            unreachable!("Fleet account has invalid FleetState discriminator")
        }
    };

    Ok((Fleet(fleet), fleet_state))
}

pub fn derive_fleet_address(
    game_pubkey: &Pubkey,
    player_profile_pubkey: &Pubkey,
    fleet_label: &str,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"Fleet",
            game_pubkey.as_ref(),
            player_profile_pubkey.as_ref(),
            &str_to_u8_32(fleet_label),
        ],
        &Sage::id(),
    )
}

pub fn get_fleet_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_pubkey: &Pubkey,
    player_profile_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(9, &game_pubkey.as_ref())),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
                41,
                &player_profile_pubkey.as_ref(),
            )),
        ]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        with_context: Some(false),
    };

    let accounts = program
        .rpc()
        .get_program_accounts_with_config(&program.id(), config)?;

    Ok(accounts)
}

pub fn get_fleet_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    fleet_pubkey: &Pubkey,
) -> anyhow::Result<Account> {
    let account = program.rpc().get_account(&fleet_pubkey)?;
    Ok(account)
}
