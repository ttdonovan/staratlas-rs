use super::*;
use crate::programs::staratlas_points::ID;

pub fn user_points_account_address(
    xp_category_pubkey: &Pubkey,
    player_profile_pubkey: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"UserPointsAccount",
            xp_category_pubkey.as_ref(),
            player_profile_pubkey.as_ref(),
        ],
        &ID,
    )
}
