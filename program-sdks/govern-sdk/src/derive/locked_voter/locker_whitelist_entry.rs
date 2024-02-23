use super::*;

use crate::LockerWhitelistEntry;

pub fn locker_whitelist_entry_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<LockerWhitelistEntry> {
    let account = program.rpc().get_account(pubkey)?;
    let locker_whitelist_entry_account = parse_locker_whitelist_entry_data(&account)?;
    Ok(locker_whitelist_entry_account)
}

fn parse_locker_whitelist_entry_data(account: &Account) -> anyhow::Result<LockerWhitelistEntry> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<LockerWhitelistEntry>(account_data)?;
    Ok(account)
}
