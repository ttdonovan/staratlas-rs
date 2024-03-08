use super::*;

use staratlas_sage::{instruction, typedefs};

use crate::{find, Fleet, Game};

// https://solscan.io/tx/37JvrXwpn9JouPPXhZJ7Z2dJUvWP9yn5unkLAHV1cT6eo6ayNnmzuckdGBkUnQijmmfA7JU7LM84V42vq7bcUvo3
pub fn warp_to_coordinate<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    cargo_program: &Program<C>,
    fleet: (&Pubkey, &Fleet),
    game: (&Pubkey, &Game),
    coordinate: [i64; 2],
) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let mut ixs = vec![];
    let (fleet_id, fleet_acct) = fleet;
    let (game_id, game_acct) = game;
    let game_state = &game_acct.0.game_state;

    // game mint
    let fuel_mint = &game_acct.0.mints.fuel;

    // cargo stats definition
    let cargo_stats_definition = &game_acct.0.cargo.stats_definition;
    // let cargo_stats_definition_acct = derive::cargo_stats_definition_account(cargo_program, cargo_stats_definition)?;
    // dbg!(&cargo_stats_definition_acct.0.seq_id);
    let seq_id = 1;

    // player profile and faction
    let player_profile = &fleet_acct.0.owner_profile;
    let (profile_faction, _) = find_profile_faction_address(&player_profile)?;

    // fleet's fuel tank and cargo type
    let fuel_tank = &fleet_acct.0.fuel_tank;
    let (fuel_cargo_type, _) = find::cargo_type_address(cargo_stats_definition, fuel_mint, seq_id);

    // token accounts
    let ata_token_from = get_associated_token_address(fuel_tank, fuel_mint);

    let instr = instruction::WarpToCoordinate {
        _input: typedefs::WarpToCoordinateInput {
            key_index: 0,
            to_sector: coordinate,
        },
    };
    let warp_to_coordinate_ix = Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(*player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(*game_state, false),
            AccountMeta::new(*fuel_tank, false),
            AccountMeta::new_readonly(fuel_cargo_type, false),
            AccountMeta::new_readonly(*cargo_stats_definition, false),
            AccountMeta::new(ata_token_from, false),
            AccountMeta::new(*fuel_mint, false),
            AccountMeta::new(cargo_program.id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
    );

    let builder = sage_program.request().instruction(warp_to_coordinate_ix);

    let ix = builder.instructions()?;
    ixs.push(ix);

    Ok(ixs)
}

pub fn ready_to_exit_warp<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, &Fleet),
) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let mut ixs = vec![];
    let (fleet_id, _) = fleet;

    let instr = instruction::FleetStateHandler {};
    let fleet_state_handler_ix = Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![AccountMeta::new(*fleet_id, false)],
    );

    let builder = sage_program.request().instruction(fleet_state_handler_ix);

    let ix = builder.instructions()?;
    ixs.push(ix);

    Ok(ixs)
}
