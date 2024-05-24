use super::*;
use crate::programs::staratlas_sage::ID;

pub fn progression_config_address(game_pubkey: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"ProgressionConfig", game_pubkey.as_ref()], &ID)
}
