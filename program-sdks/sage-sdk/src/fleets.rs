use anchor_client::{
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::signature::Signer,
    Program,
};
use anchor_lang::{prelude::Pubkey, Id};

use staratlas_sage::{program::Sage, state};

use crate::{utils::str_to_u8_32, Fleet};

use std::ops::Deref;

pub fn derive_fleet_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    player_profile_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Fleet)>> {
    let accounts = program.accounts::<state::Fleet>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(41, &player_profile_pubkey.to_bytes()),
    )])?;

    let fleet_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, Fleet(account.clone())))
        .collect();

    Ok(fleet_accounts)
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
