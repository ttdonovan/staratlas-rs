use super::*;

use crate::Proxy;

pub fn proxy_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<Proxy> {
    let account = program.rpc().get_account(pubkey)?;
    let proxy_account = parse_proxy_data(&account)?;
    Ok(proxy_account)
}

pub fn proxy_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Proxy)>> {
    let accounts = get_proxy_accounts(program, owner)?;

    let proxy_accounts: Vec<_> = accounts
        .iter()
        .filter_map(|(pubkey, account)| match parse_proxy_data(&account) {
            Ok(data) => Some((*pubkey, data)),
            Err(_) => None,
        })
        .collect();

    Ok(proxy_accounts)
}

pub fn get_proxy_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::DataSize(137),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(40, owner.as_ref())),
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

fn parse_proxy_data(account: &Account) -> anyhow::Result<Proxy> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<Proxy>(account_data)?;
    Ok(account)
}
