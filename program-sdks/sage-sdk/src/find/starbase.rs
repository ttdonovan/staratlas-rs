use super::*;
use crate::programs::staratlas_sage::program::Sage;

pub fn starbase_address(game_id: &Pubkey, sector_coordinates: [i64; 2]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"Starbase",
            game_id.as_ref(),
            &sector_coordinates[0].to_le_bytes(),
            &sector_coordinates[1].to_le_bytes(),
        ],
        &Sage::id(),
    )
}

pub fn starbase_player_address(
    starbase: &Pubkey,
    sage_player_profile: &Pubkey,
    seq_id: u16,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"starbase_player",
            starbase.as_ref(),
            sage_player_profile.as_ref(),
            &seq_id.to_le_bytes(),
        ],
        &Sage::id(),
    )
}
