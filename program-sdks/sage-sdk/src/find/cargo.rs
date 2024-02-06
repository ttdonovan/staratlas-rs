use super::*;
use crate::programs::staratlas_cargo::program::Cargo;

pub fn cargo_type_address(stats_definition: &Pubkey, mint: &Pubkey, seq_id: u16) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"cargo_type",
            &seq_id.to_le_bytes(),
            stats_definition.as_ref(),
            mint.as_ref(),
        ],
        &Cargo::id(),
    )
}
