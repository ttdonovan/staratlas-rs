use anchor_client::{
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{
        account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer,
    },
    Program,
};
use anchor_lang::{AnchorDeserialize, Id};
use solana_account_decoder::UiAccountEncoding;

use std::ops::Deref;

mod cargo;
mod fleet;
mod game;
mod planet;

pub use cargo::*;
pub use fleet::*;
pub use game::*;
pub use planet::*;

pub fn derive_account<
    C: Deref<Target = impl Signer> + Clone,
    T: anchor_lang::AccountDeserialize,
>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<T> {
    let account = program.account::<T>(*pubkey)?;
    Ok(account)
}
