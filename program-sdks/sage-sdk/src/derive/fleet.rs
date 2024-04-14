use super::*;

use staratlas_sage::{state, typedefs, ID};

use crate::{accounts, utils::str_to_u8_32};

pub fn fleet_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_pubkey: &Pubkey,
    player_profile_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, accounts::Fleet)>> {
    let accounts = program.accounts::<state::Fleet>(vec![
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(9, game_pubkey.as_ref())),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            41,
            player_profile_pubkey.as_ref(),
        )),
    ])?;

    let fleet_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, accounts::Fleet::from(*account)))
        .collect();

    Ok(fleet_accounts)
}

pub fn fleet_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    fleet_pubkey: &Pubkey,
) -> anyhow::Result<accounts::Fleet> {
    let account = program.account::<state::Fleet>(*fleet_pubkey)?;
    Ok(account.into())
}

pub fn fleet_account_with_state<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    fleet_pubkey: &Pubkey,
) -> anyhow::Result<(accounts::Fleet, accounts::FleetState)> {
    let account = get_fleet_account(program, fleet_pubkey)?;
    fleet_with_state(&account)
}

pub fn fleet_with_state(
    account: &Account,
) -> anyhow::Result<(accounts::Fleet, accounts::FleetState)> {
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
            accounts::FleetState::StarbaseLoadingBay(starbase_loading_bay.into())
        }
        1 => {
            let idle = typedefs::Idle::deserialize(&mut remaining_data)?;
            accounts::FleetState::Idle(idle.into())
        }
        2 => {
            let mine_astriod = typedefs::MineAsteroid::deserialize(&mut remaining_data)?;
            accounts::FleetState::MineAsteroid(mine_astriod.into())
        }
        3 => {
            let move_warp = typedefs::MoveWarp::deserialize(&mut remaining_data)?;
            accounts::FleetState::MoveWarp(move_warp.into())
        }
        4 => {
            let move_subwarp = typedefs::MoveSubwarp::deserialize(&mut remaining_data)?;
            accounts::FleetState::MoveSubwarp(move_subwarp.into())
        }
        5 => {
            let respawn = typedefs::Respawn::deserialize(&mut remaining_data)?;
            accounts::FleetState::Respawn(respawn.into())
        }
        _ => {
            unreachable!("Fleet account has invalid FleetState discriminator")
        }
    };

    Ok((fleet.into(), fleet_state))
}

pub fn fleet_address(
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
        &ID,
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
