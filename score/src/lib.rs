use anchor_lang::prelude::Pubkey;

anchor_gen::generate_cpi_crate!("idl.json");

declare_id!("FLEET1qqzpexyaDpqb2DGsSzE2sDCizewCg9WjrA6DBW");

const SCORE_INFO_SEED: &[u8; 10] = b"SCORE_INFO";
const SCOREVARS_SHIP_SEED: &[u8; 14] = b"SCOREVARS_SHIP";

pub fn get_score_vars_ship_account(ship_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SCOREVARS_SHIP_SEED, &ship_mint.to_bytes()], &ID)
}

pub fn get_ship_staking_account(player_pubkey: &Pubkey, ship_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            SCORE_INFO_SEED,
            &player_pubkey.to_bytes(),
            &ship_mint.to_bytes(),
        ],
        &ID,
    )
}
