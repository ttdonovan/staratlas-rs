use anchor_client::anchor_lang::prelude::Pubkey;

use crate::utils::str_to_u8_32;

use std::str::FromStr;

use staratlas_cargo::ID as CARGO_ID;
use staratlas_points::ID as POINTS_ID;
use staratlas_sage::ID as SAGE_ID;

pub fn cargo_type_address(stats_definition: &Pubkey, mint: &Pubkey, seq_id: u16) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"cargo_type",
            seq_id.to_le_bytes().as_ref(),
            stats_definition.as_ref(),
            mint.as_ref(),
        ],
        &CARGO_ID,
    )
}

pub fn fleet_address(game: &Pubkey, player_profile: &Pubkey, fleet_label: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"Fleet",
            game.as_ref(),
            player_profile.as_ref(),
            &str_to_u8_32(fleet_label),
        ],
        &SAGE_ID,
    )
}

pub fn profile_faction_address(player_profile: &Pubkey) -> (Pubkey, u8) {
    let program_id = Pubkey::from_str("pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq")
        .expect("Invalid program ID");
    Pubkey::find_program_address(&[b"player_faction", player_profile.as_ref()], &program_id)
}

pub fn progression_config_address(game_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"ProgressionConfig", game_id.as_ref()], &SAGE_ID)
}

pub fn sage_player_profile_address(game_id: &Pubkey, player_profile: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"sage_player_profile",
            player_profile.as_ref(),
            game_id.as_ref(),
        ],
        &SAGE_ID,
    )
}

pub fn starbase_address(game_id: &Pubkey, sector_coordinates: [i64; 2]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"Starbase",
            game_id.as_ref(),
            &sector_coordinates[0].to_le_bytes(),
            &sector_coordinates[1].to_le_bytes(),
        ],
        &SAGE_ID,
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
        &SAGE_ID,
    )
}

pub fn user_points_account_address(xp_category: &Pubkey, player_profile: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"UserPointsAccount",
            xp_category.as_ref(),
            player_profile.as_ref(),
        ],
        &POINTS_ID,
    )
}
