use super::*;

use crate::ProxyEscrow;

pub fn proxy_escrow_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<ProxyEscrow> {
    let account = program.rpc().get_account(pubkey)?;
    let proxy_escrow_account = parse_proxy_escrow_data(&account)?;
    Ok(proxy_escrow_account)
}

pub fn proxy_escrow_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    escrow_owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, ProxyEscrow)>> {
    let accounts = get_proxy_escrow_accounts(program, escrow_owner)?;

    let proxy_escrow_accounts: Vec<_> = accounts
        .iter()
        .filter_map(
            |(pubkey, account)| match parse_proxy_escrow_data(&account) {
                Ok(data) => Some((*pubkey, data)),
                Err(_) => None,
            },
        )
        .collect();

    Ok(proxy_escrow_accounts)
}

pub fn get_proxy_escrow_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    escrow_owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::DataSize(81),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(8, escrow_owner.as_ref())),
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

fn parse_proxy_escrow_data(account: &Account) -> anyhow::Result<ProxyEscrow> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<ProxyEscrow>(account_data)?;
    Ok(account)
}
