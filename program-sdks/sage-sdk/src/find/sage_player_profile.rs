use super::*;
use crate::programs::staratlas_sage::ID;

pub fn sage_player_profile_address(game_id: &Pubkey, player_profile: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"sage_player_profile",
            player_profile.as_ref(),
            game_id.as_ref(),
        ],
        &ID,
    )
}
