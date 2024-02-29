use super::*;

use staratlas_sage::state;

use crate::Starbase;

pub fn starbase_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    starbase_pubkey: &Pubkey,
) -> anyhow::Result<Starbase> {
    let account = program.account::<state::Starbase>(*starbase_pubkey)?;
    Ok(Starbase(account))
}
