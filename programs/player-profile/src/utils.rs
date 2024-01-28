use anchor_client::{
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, signature::Signer},
    Program,
};
use anchor_lang::prelude::Pubkey;
use solana_account_decoder::UiAccountEncoding;

use std::ops::Deref;

use crate::state::Profile;

pub fn derive_profile_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    player_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Profile)>> {
    let profile_accounts = program.accounts::<Profile>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(30, &player_pubkey.to_bytes()),
    )])?;

    Ok(profile_accounts)
}

pub fn get_profile_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    player_pubkey: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            30,
            &player_pubkey.to_bytes(),
        ))]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        with_context: Some(false),
    };

    let profile_accounts = program
        .rpc()
        .get_program_accounts_with_config(&program.id(), config);

    Ok(profile_accounts?)
}
