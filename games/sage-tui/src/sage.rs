use anchor_client::{solana_sdk::signature::Keypair, Client, Program};
use anyhow::Result;

use std::rc::Rc;

use staratlas_player_profile_sdk::{
    derive::profile_accounts as derive_profile_accounts,
    programs::staratlas_player_profile::ID as PROFILE_ID, Profile,
};
use staratlas_sage_sdk::{
    derive,
    programs::{staratlas_cargo::ID as CARGO_ID, staratlas_sage::ID as SAGE_ID},
};

pub use anchor_client::solana_sdk::pubkey::Pubkey;
pub use staratlas_sage_sdk::accounts::{Fleet, FleetState, Game};

pub fn list_games(client: &Client<Rc<Keypair>>) -> Result<Vec<(Pubkey, Game)>> {
    let program = client.program(SAGE_ID)?;
    let games = derive::game_accounts(&program)?;
    Ok(games)
}

pub fn list_player_profiles(client: &Client<Rc<Keypair>>) -> Result<Vec<(Pubkey, Profile)>> {
    let program = client.program(PROFILE_ID)?;
    let player_profiles = derive_profile_accounts(&program, &program.payer())?;
    Ok(player_profiles)
}

pub struct SageContext {
    _client: Client<Rc<Keypair>>,
    sage_program: Program<Rc<Keypair>>,
    _cargo_program: Program<Rc<Keypair>>,
    pub game_id: Pubkey,
    _game: Game,
    pub profile_id: Pubkey,
    _profile: Profile,
}

impl SageContext {
    pub fn new(
        game: (Pubkey, Game),
        profile: (Pubkey, Profile),
        client: Client<Rc<Keypair>>,
    ) -> Result<Self> {
        let sage_program = client.program(SAGE_ID)?;
        let cargo_program = client.program(CARGO_ID)?;

        Ok(Self {
            _client: client,
            sage_program,
            _cargo_program: cargo_program,
            game_id: game.0,
            _game: game.1,
            profile_id: profile.0,
            _profile: profile.1,
        })
    }

    pub fn get_fleet_with_state(&self) -> Vec<(Pubkey, (Fleet, FleetState))> {
        let program = &self.sage_program;
        let fleet = derive::get_fleet_accounts(&program, &self.game_id, &self.profile_id).unwrap();
        fleet
            .iter()
            .filter_map(
                |(pubkey, account)| match derive::fleet_with_state(&account) {
                    Ok(fleet) => Some((*pubkey, fleet)),
                    Err(_) => None,
                },
            )
            .collect()
    }
}
