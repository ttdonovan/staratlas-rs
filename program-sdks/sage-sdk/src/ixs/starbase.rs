use super::*;

use staratlas_sage::{instruction, state};

use crate::{
    accounts::{Fleet, FleetState, Game},
    find,
};

pub fn dock_to_starbase<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, (&Fleet, &FleetState)),
    game: (&Pubkey, &Game),
) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let mut ixs = vec![];
    let (fleet_pubkey, (fleet, fleet_state)) = fleet;
    let (game_pubkey, game) = game;

    match fleet_state {
        FleetState::Idle(idle) => {
            let fleet_id = fleet_pubkey;
            let player_profile = &fleet.owner_profile;
            let game_id = game_pubkey;
            let game_state = &game.game_state;

            let (starbase, _) = find::starbase_address(game_id, idle.sector);
            let starbase_acct = derive_account::<_, state::Starbase>(sage_program, &starbase)?;
            let (profile_faction, _) = find_profile_faction_address(player_profile)?;
            let (sage_player_profile, _) =
                find::sage_player_profile_address(game_id, player_profile);
            let (starbase_player, _) = find::starbase_player_address(
                &starbase,
                &sage_player_profile,
                starbase_acct.seq_id,
            );

            let instr = instruction::IdleToLoadingBay { _key_index: 0 };
            let idle_to_loading_bay_ix = Instruction::new_with_bytes(
                sage_program.id(),
                &instr.data(),
                vec![
                    AccountMeta::new(sage_program.payer(), true),
                    AccountMeta::new_readonly(*player_profile, false),
                    AccountMeta::new_readonly(profile_faction, false),
                    AccountMeta::new(*fleet_id, false),
                    AccountMeta::new_readonly(*game_id, false),
                    AccountMeta::new_readonly(*game_state, false),
                    AccountMeta::new_readonly(starbase, false),
                    AccountMeta::new(starbase_player, false),
                ],
            );

            let builder = sage_program.request().instruction(idle_to_loading_bay_ix);
            let ix = builder.instructions()?;
            ixs.push(ix);
        }
        _ => {}
    }

    Ok(ixs)
}

pub fn undock_from_starbase<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, (&Fleet, &FleetState)),
    game: (&Pubkey, &Game),
) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let mut ixs = vec![];
    let (fleet_pubkey, (fleet, fleet_state)) = fleet;
    let (game_pubkey, game) = game;

    match fleet_state {
        FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
            let fleet_id = fleet_pubkey;
            let player_profile = &fleet.owner_profile;
            let game_id = game_pubkey;
            let game_state = &game.game_state;

            let starbase = starbase_loading_bay.starbase;
            let starbase_acct = derive_account::<_, state::Starbase>(sage_program, &starbase)?;
            let (profile_faction, _) = find_profile_faction_address(player_profile)?;
            let (sage_player_profile, _) =
                find::sage_player_profile_address(game_id, player_profile);
            let (starbase_player, _) = find::starbase_player_address(
                &starbase,
                &sage_player_profile,
                starbase_acct.seq_id,
            );

            let instr = instruction::LoadingBayToIdle { _key_index: 0 };
            let loading_bay_to_idle_ix = Instruction::new_with_bytes(
                sage_program.id(),
                &instr.data(),
                vec![
                    AccountMeta::new(sage_program.payer(), true),
                    AccountMeta::new_readonly(*player_profile, false),
                    AccountMeta::new_readonly(profile_faction, false),
                    AccountMeta::new(*fleet_id, false),
                    AccountMeta::new_readonly(*game_id, false),
                    AccountMeta::new_readonly(*game_state, false),
                    AccountMeta::new_readonly(starbase, false),
                    AccountMeta::new(starbase_player, false),
                    AccountMeta::new(starbase, false),
                ],
            );

            let builder = sage_program.request().instruction(loading_bay_to_idle_ix);
            let ix = builder.instructions()?;
            ixs.push(ix);
        }
        _ => {}
    }

    Ok(ixs)
}
