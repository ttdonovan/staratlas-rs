use super::*;

use crate::Locker;

pub fn locker_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<Locker> {
    let account = program.rpc().get_account(pubkey)?;
    let locker_account = parse_locker_data(&account)?;
    Ok(locker_account)
}

fn parse_locker_data(account: &Account) -> anyhow::Result<Locker> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<Locker>(account_data)?;
    Ok(account)
}
