use super::*;

use crate::Escrow;

pub fn escrow_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    pubkey: &Pubkey,
) -> anyhow::Result<Escrow> {
    let account = program.rpc().get_account(pubkey)?;
    let escrow_account = parse_escrow_data(&account)?;
    Ok(escrow_account)
}

pub fn escrow_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Escrow)>> {
    let accounts = get_escrow_accounts(program, owner)?;

    dbg!(&accounts);

    Ok(vec![])
}

pub fn get_escrow_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    owner: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Account)>> {
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![
            // RpcFilterType::DataSize(171),
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

fn parse_escrow_data(account: &Account) -> anyhow::Result<Escrow> {
    let account_data = account.data.as_slice();
    let account_data = &account_data[8..];
    let account = borsh::from_slice::<Escrow>(account_data)?;
    Ok(account)
}
