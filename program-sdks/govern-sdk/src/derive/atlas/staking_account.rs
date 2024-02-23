use super::*;

use crate::StakingAccount;

pub fn staking_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, StakingAccount)>> {
    let accounts = get_staking_accounts(program, owner)?;

    let staking_accounts: Vec<_> = accounts
        .iter()
        .filter_map(
            |(pubkey, account)| match parse_staking_account_data(&account) {
                Ok(data) => Some((*pubkey, data)),
                Err(_) => None,
            },
        )
        .collect();

    Ok(staking_accounts)
}

pub fn get_staking_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            RpcFilterType::DataSize(171),
            RpcFilterType::Memcmp(Memcmp::new_base58_encoded(8, owner.as_ref())),
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

fn parse_staking_account_data(account: &Account) -> anyhow::Result<StakingAccount> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<StakingAccount>(account_data)?;
    Ok(account)
}
