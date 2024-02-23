use super::*;

use crate::RegisteredLocker;

pub fn registered_locker_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<RegisteredLocker> {
    let account = program.rpc().get_account(pubkey)?;
    let registered_locker_account = parse_registered_locker_data(&account)?;
    Ok(registered_locker_account)
}

pub fn registered_locker_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    locker: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, RegisteredLocker)>> {
    let accounts = get_registered_locker_accounts(program, locker)?;

    let registered_locker_accounts: Vec<_> = accounts
        .iter()
        .filter_map(
            |(pubkey, account)| match parse_registered_locker_data(&account) {
                Ok(data) => Some((*pubkey, data)),
                Err(_) => None,
            },
        )
        .collect();

    Ok(registered_locker_accounts)
}

pub fn get_registered_locker_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    locker: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::DataSize(6256),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(73, locker.as_ref())),
        ]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::confirmed()),
            ..Default::default()
        },
        with_context: Some(false),
    };

    let accounts = program
        .rpc()
        .get_program_accounts_with_config(&program.id(), config)?;

    Ok(accounts)
}

fn parse_registered_locker_data(account: &Account) -> anyhow::Result<RegisteredLocker> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<RegisteredLocker>(account_data)?;
    Ok(account)
}
