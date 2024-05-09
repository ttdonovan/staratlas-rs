use anchor_client::{
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::{pubkey::Pubkey, signature::Signer},
    ClientError, Program,
};

use crate::accounts::{Fleet, Planet};
use staratlas_sage::state;

use std::ops::Deref;

pub async fn fleets_by_game_and_player_profile<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_id: &Pubkey,
    player_profile_id: &Pubkey,
) -> Result<Vec<(Pubkey, Fleet)>, ClientError> {
    let accounts = program
        .accounts::<state::Fleet>(vec![
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(9, game_id.as_ref())),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(41, player_profile_id.as_ref())),
        ])
        .await?;

    let accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, Fleet::from(*account)))
        .collect();

    Ok(accounts)
}

pub async fn planets_by_game_and_coords<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_id: &Pubkey,
    sector_coordinates: [i64; 2],
) -> Result<Vec<(Pubkey, Planet)>, ClientError> {
    let accounts = program
        .accounts::<state::Planet>(vec![
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(73, game_id.as_ref())),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
                105,
                &sector_coordinates[0].to_le_bytes(),
            )),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
                113,
                &sector_coordinates[1].to_le_bytes(),
            )),
        ])
        .await?;

    let accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, Planet::from(*account)))
        .collect();

    Ok(accounts)
}
