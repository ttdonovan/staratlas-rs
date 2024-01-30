use anchor_client::{
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::signature::Signer,
    Program,
};
use anchor_lang::{prelude::Pubkey, Id};

use staratlas_sage::{program::Sage, state};

use crate::{Fleet, Game};

use std::ops::Deref;

fn str_to_u8_32(s: &str) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        array[i] = byte;
    }

    array
}

pub fn derive_fleet_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    player_profile_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Fleet)>> {
    let accounts = program.accounts::<state::Fleet>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(41, &player_profile_pubkey.to_bytes()),
    )])?;

    let fleet_accounts = accounts
        .into_iter()
        .fold(Vec::new(), |mut acc, (pubkey, fleet)| {
            acc.push((pubkey, Fleet(fleet)));
            acc
        });

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

pub fn derive_game_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
) -> anyhow::Result<Vec<(Pubkey, Game)>> {
    let accounts = program.accounts::<state::Game>(vec![])?;

    let game_accounts = accounts
        .into_iter()
        .fold(Vec::new(), |mut acc, (pubkey, game)| {
            acc.push((pubkey, Game(game)));
            acc
        });

    Ok(game_accounts)
}
