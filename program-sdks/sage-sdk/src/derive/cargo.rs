use super::*;
use crate::{programs::staratlas_cargo::state, CargoStatsDefinition, CargoType};

pub fn cargo_stats_definition_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    cargo_stats_definition_pubkey: &Pubkey,
) -> anyhow::Result<CargoStatsDefinition> {
    let acct =
        derive_account::<_, state::CargoStatsDefinition>(program, cargo_stats_definition_pubkey)?;
    Ok(CargoStatsDefinition(acct))
}

pub fn cargo_type_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    cargo_stats_definition_pubkey: &Pubkey,
    mint: &Pubkey,
    seq_id: u16,
) -> anyhow::Result<Vec<(Pubkey, CargoType)>> {
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
        .map(|(pubkey, account)| (*pubkey, CargoType(account.clone())))
        .collect();

    Ok(cargo_type_accounts)
}
