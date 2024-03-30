use super::*;
use crate::{accounts, programs::staratlas_sage::state};

pub fn planet_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_id: &Pubkey,
    sector_coordinates: [i64; 2],
) -> anyhow::Result<Vec<(Pubkey, accounts::Planet)>> {
    let accounts = program.accounts::<state::Planet>(vec![
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(73, game_id.as_ref())),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            105,
            &sector_coordinates[0].to_le_bytes(),
        )),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
            113,
            &sector_coordinates[1].to_le_bytes(),
        )),
    ])?;

    let planet_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, accounts::Planet::from(*account)))
        .collect();

    Ok(planet_accounts)
}

pub fn resource_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_id: &Pubkey,
    location: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, accounts::Resource)>> {
    let accounts = program.accounts::<state::Resource>(vec![
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(9, game_id.as_ref())),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(41, location.as_ref())),
    ])?;

    let resource_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, accounts::Resource::from(*account)))
        .collect();

    Ok(resource_accounts)
}
