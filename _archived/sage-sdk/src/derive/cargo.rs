use super::*;
use crate::{accounts, programs::staratlas_cargo::state};

pub fn cargo_pod_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    starbase_player: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, accounts::CargoPod)>> {
    let accounts = program.accounts::<state::CargoPod>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(41, starbase_player.as_ref()),
    )])?;

    let cargo_pod_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, accounts::CargoPod::from(*account)))
        .collect();

    Ok(cargo_pod_accounts)
}

pub fn cargo_stats_definition_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    cargo_stats_definition_pubkey: &Pubkey,
) -> anyhow::Result<accounts::CargoStatsDefinition> {
    let account =
        derive_account::<_, state::CargoStatsDefinition>(program, cargo_stats_definition_pubkey)?;
    Ok(account.into())
}

pub fn cargo_type_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    cargo_stats_definition_pubkey: &Pubkey,
    mint: &Pubkey,
    seq_id: u16,
) -> anyhow::Result<Vec<(Pubkey, accounts::CargoType)>> {
    let accounts = program.accounts::<state::CargoType>(vec![
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            9,
            cargo_stats_definition_pubkey.as_ref(),
        )),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(41, mint.as_ref())),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(76, &seq_id.to_le_bytes())),
    ])?;

    let cargo_type_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, accounts::CargoType::from(*account)))
        .collect();

    Ok(cargo_type_accounts)
}
