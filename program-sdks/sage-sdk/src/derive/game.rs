use super::*;

use staratlas_sage::state;

use crate::accounts;

pub fn game_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_pubkey: &Pubkey,
) -> anyhow::Result<accounts::Game> {
    let account = program.account::<state::Game>(*game_pubkey)?;
    Ok(account.into())
}

pub fn game_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
) -> anyhow::Result<Vec<(Pubkey, accounts::Game)>> {
    let accounts = program.accounts::<state::Game>(vec![])?;

    let game_accounts = accounts
        .into_iter()
        .fold(Vec::new(), |mut acc, (pubkey, game)| {
            acc.push((pubkey, game.into()));
            acc
        });

    Ok(game_accounts)
}

pub fn game_state_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_state_pubkey: &Pubkey,
) -> anyhow::Result<accounts::GameState> {
    let account = program.account::<state::GameState>(*game_state_pubkey)?;
    Ok(account.into())
}
