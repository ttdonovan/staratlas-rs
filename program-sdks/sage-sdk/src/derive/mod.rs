use anchor_client::{
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::{pubkey::Pubkey, signature::Signer},
    Program,
};

use std::ops::Deref;

mod cargo;
mod planet;

pub use cargo::*;
pub use planet::*;

pub(crate) fn derive_account<
    C: Deref<Target = impl Signer> + Clone,
    T: anchor_lang::AccountDeserialize,
>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<T> {
    let account = program.account::<T>(*pubkey)?;
    Ok(account)
}
