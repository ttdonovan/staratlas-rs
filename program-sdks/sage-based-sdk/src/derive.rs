use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::signature::Signer,
    ClientError, Program,
};

use staratlas_cargo::state as cargo_state;

use crate::accounts::CargoPod;

use std::ops::Deref;

pub async fn cargo_pod_accounts<C: Deref<Target = impl Signer> + Clone>(
    cargo_program: &Program<C>,
    starbase_player: &Pubkey,
) -> Result<Vec<(Pubkey, CargoPod)>, ClientError> {
    let accounts = cargo_program
        .accounts::<cargo_state::CargoPod>(vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            41,
            starbase_player.as_ref(),
        ))])
        .await?;

    let cargo_pod_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, CargoPod::from(*account)))
        .collect();

    Ok(cargo_pod_accounts)
}
