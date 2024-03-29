use super::*;

use staratlas_sage::state;

use crate::{Game, GameState};

pub fn game_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_pubkey: &Pubkey,
) -> anyhow::Result<Game> {
    let account = program.account::<state::Game>(*game_pubkey)?;
    Ok(Game(account))
}

pub fn game_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
) -> anyhow::Result<Vec<(Pubkey, Game)>> {
    let accounts = program.accounts::<state::Game>(vec![])?;

    let game_accounts = accounts
        .into_iter()
        .fold(Vec::new(), |mut acc, (pubkey, game)| {
            acc.push((pubkey, Game(game)));
            acc
        });

    Ok(game_accounts)
}

pub fn game_state_account<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    game_state_pubkey: &Pubkey,
) -> anyhow::Result<GameState> {
    let account = program.account::<state::GameState>(*game_state_pubkey)?;
    Ok(GameState(account))
}
