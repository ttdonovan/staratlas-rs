use anchor_client::{
    solana_sdk::{instruction::Instruction, signature::Signer},
    Program,
};
use anchor_lang::{
    prelude::{AccountMeta, Pubkey},
    Id, InstructionData,
};

use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};

use std::ops::Deref;
use std::str::FromStr;

pub mod cargo;
pub mod fleet;
pub mod mining;

fn derive_account<C: Deref<Target = impl Signer> + Clone, T: anchor_lang::AccountDeserialize>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<T> {
    let account = program.account::<T>(*pubkey)?;
    Ok(account)
}

fn find_profile_faction_address(player_profile_pubkey: &Pubkey) -> anyhow::Result<(Pubkey, u8)> {
    let program_id = Pubkey::from_str("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq")?;
    Ok(Pubkey::find_program_address(
        &[b"player_faction", player_profile_pubkey.as_ref()],
        &program_id,
    ))
}
