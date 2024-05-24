use super::*;

use staratlas_cargo::program::Cargo2 as Cargo;
use staratlas_points::program::Points;
use staratlas_sage::{instruction, state, typedefs};

use crate::{
    accounts::{Fleet, FleetState, Game},
    derive, find,
};

pub fn start_mining_asteroid<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, (&Fleet, &FleetState)),
    game: (&Pubkey, &Game),
    mine_item: Option<Pubkey>,
) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let mut ixs = vec![];
    let (fleet_pubkey, (fleet, fleet_state)) = fleet;
    let (game_pubkey, game) = game;

    match fleet_state {
        FleetState::Idle(idle) => {
            let fleet_id = fleet_pubkey;
            let player_profile = fleet.owner_profile;
            let (profile_faction, _) = find_profile_faction_address(&player_profile)?;
            let game_id = game_pubkey;
            let game_state = &game.game_state;
            let ata_fleet_fuel = get_associated_token_address(&fleet.fuel_tank, &game.mints.fuel);

            let (starbase, _) = find::starbase_address(game_id, idle.sector);
            let starbase_acct = derive_account::<_, state::Starbase>(sage_program, &starbase)?;

            let (sage_player_profile, _) =
                find::sage_player_profile_address(game_id, &player_profile);
            let (starbase_player, _) = find::starbase_player_address(
                &starbase,
                &sage_player_profile,
                starbase_acct.seq_id,
            );

            let planets = derive::planet_accounts(sage_program, game_id, starbase_acct.sector)?;
            let (planet, _) = planets
                .into_iter()
                .find(|(_, planet)| planet.num_resources >= 1)
                .expect("planet with resources");

            let resources = derive::resource_accounts(sage_program, game_id, &planet)?;
            let (resource, resource_acct) = match mine_item {
                Some(mine_item) => resources
                    .iter()
                    .find(|(_, resource)| resource.mine_item == mine_item)
                    .expect("resource with mine item"),
                None => resources.first().expect("at least one resource"),
            };
            let mine_item = &resource_acct.mine_item;

            let instr = instruction::StartMiningAsteroid {
                _input: typedefs::KeyIndexInput { key_index: 0 },
            };
            let start_mining_asteroid_ix = Instruction::new_with_bytes(
                sage_program.id(),
                &instr.data(),
                vec![
                    AccountMeta::new(sage_program.payer(), true),
                    AccountMeta::new_readonly(player_profile, false),
                    AccountMeta::new_readonly(profile_faction, false),
                    AccountMeta::new(*fleet_id, false),
                    AccountMeta::new_readonly(*game_id, false),
                    AccountMeta::new_readonly(*game_state, false),
                    AccountMeta::new(ata_fleet_fuel, false),
                    AccountMeta::new_readonly(starbase, false),
                    AccountMeta::new_readonly(starbase_player, false),
                    AccountMeta::new_readonly(*mine_item, false),
                    AccountMeta::new(*resource, false),
                    AccountMeta::new(planet, false),
                ],
            );

            let builder = sage_program.request().instruction(start_mining_asteroid_ix);

            let ix = builder.instructions()?;
            ixs.push(ix);
        }
        _ => {}
    }

    Ok(ixs)
}

pub fn stop_mining_asteroid<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, (&Fleet, &FleetState)),
    game: (&Pubkey, &Game),
) -> anyhow::Result<Vec<Vec<Instruction>>> {
    let mut ixs = vec![];
    let (fleet_pubkey, (fleet, fleet_state)) = fleet;
    let (game_pubkey, game) = game;

    match fleet_state {
        FleetState::MineAsteroid(mine_asteroid) => {
            let fleet_id = fleet_pubkey;
            let player_profile = fleet.owner_profile;
            let game_id = game_pubkey;
            let game_state = &game.game_state;

            // game mints
            let ammo_mint = &game.mints.ammo;
            let food_mint = &game.mints.food;
            let fuel_mint = &game.mints.fuel;

            // cargo stats definition
            let cargo_stats_definition = &game.cargo.stats_definition;
            // let cargo_stats_definition_acct = derive::cargo_stats_definition_account(cargo_program, cargo_stats_definition)?;
            // dbg!(&cargo_stats_definition_acct.0.seq_id);
            let seq_id = 0;

            // player profile's faction
            let (profile_faction, _) = find_profile_faction_address(&player_profile)?;

            // mine asteroid's resource (account), mine item (account), planet (account)
            let resource =
                derive_account::<_, state::Resource>(sage_program, &mine_asteroid.resource)?;
            let mine_item =
                derive_account::<_, state::MineItem>(sage_program, &resource.mine_item)?;
            let planet = derive_account::<_, state::Planet>(sage_program, &mine_asteroid.asteroid)?;

            let (starbase, _) = find::starbase_address(game_id, planet.sector);
            let resource_mint = &mine_item.mint;
            let mine_item = &resource.mine_item;
            let planet = &resource.location;
            let resource = &mine_asteroid.resource;

            // fleet's cargo (food), ammo and fuel
            let cargo_hold = &fleet.cargo_hold;
            let ammo_bank = &fleet.ammo_bank;
            let fuel_tank = &fleet.fuel_tank;

            // ata for resource
            let ata_resource_from = get_associated_token_address(&mine_item, resource_mint);
            let ata_resource_to = get_associated_token_address(cargo_hold, resource_mint);

            let ata_resource_to_ix = create_associated_token_account_idempotent(
                &sage_program.payer(),
                &ata_resource_to,
                resource_mint,
                &spl_token::id(),
            );

            // ata for ammo, food and fuel
            let ata_fleet_ammo = get_associated_token_address(ammo_bank, ammo_mint);
            let ata_fleet_food = get_associated_token_address(cargo_hold, food_mint);
            let ata_fleet_fuel = get_associated_token_address(fuel_tank, fuel_mint);

            let food_token_from = &ata_fleet_food;
            let ammo_token_from = &ata_fleet_ammo;

            // let ata_ammo_from_ix = create_associated_token_account_idempotent(
            //     &sage_program.payer(),
            //     &ammo_token_from,
            //     ammo_mint,
            //     &spl_token::id(),
            // );

            // let ata_food_from_ix = create_associated_token_account_idempotent(
            //     &sage_program.payer(),
            //     &food_token_from,
            //     food_mint,
            //     &spl_token::id(),
            // );

            let resource_token_from = ata_resource_from;
            let resource_token_to = ata_resource_to;

            let (food_cargo_type, _) =
                find::cargo_type_address(cargo_stats_definition, food_mint, seq_id);
            let (ammo_cargo_type, _) =
                find::cargo_type_address(cargo_stats_definition, ammo_mint, seq_id);
            let (fuel_cargo_type, _) =
                find::cargo_type_address(cargo_stats_definition, fuel_mint, seq_id);
            let (resource_cargo_type, _) =
                find::cargo_type_address(cargo_stats_definition, resource_mint, seq_id);

            let pilot_points_category_pubkey = game.points.pilot_xp_category.category;
            let pilot_points_modifer_pubkey = game.points.pilot_xp_category.modifier;
            let (pilot_user_points, _) =
                find::user_points_account_address(&pilot_points_category_pubkey, &player_profile);

            let mining_points_category_pubkey = game.points.mining_xp_category.category;
            let mining_points_modifier_pubkey = game.points.mining_xp_category.modifier;
            let (mining_user_points, _) =
                find::user_points_account_address(&mining_points_category_pubkey, &player_profile);

            let council_rank_points_category_pubkey = game.points.council_rank_xp_category.category;
            let council_rank_points_modifier_pubkey = game.points.council_rank_xp_category.modifier;
            let (council_rank_user_points, _) = find::user_points_account_address(
                &council_rank_points_category_pubkey,
                &player_profile,
            );

            let (progress_config, _) = find::progression_config_address(&game_id);

            let instr = instruction::FleetStateHandler {};
            let fleet_state_handler_ix = Instruction::new_with_bytes(
                sage_program.id(),
                &instr.data(),
                vec![
                    AccountMeta::new(*fleet_id, false),
                    AccountMeta::new(*cargo_hold, false),
                    AccountMeta::new(*ammo_bank, false),
                    AccountMeta::new_readonly(*mine_item, false),
                    AccountMeta::new(*resource, false),
                    AccountMeta::new(*planet, false),
                    AccountMeta::new_readonly(starbase, false),
                    AccountMeta::new(*food_token_from, false),
                    AccountMeta::new(*ammo_token_from, false),
                    AccountMeta::new(resource_token_from, false),
                    AccountMeta::new(resource_token_to, false),
                    AccountMeta::new(*food_mint, false),
                    AccountMeta::new(*ammo_mint, false),
                    AccountMeta::new_readonly(food_cargo_type, false),
                    AccountMeta::new_readonly(ammo_cargo_type, false),
                    AccountMeta::new_readonly(resource_cargo_type, false),
                    AccountMeta::new_readonly(*cargo_stats_definition, false),
                    AccountMeta::new_readonly(*game_state, false),
                    AccountMeta::new_readonly(*game_id, false),
                    AccountMeta::new_readonly(Cargo::id(), false),
                    AccountMeta::new_readonly(spl_token::id(), false),
                ],
            );

            let instr = instruction::StopMiningAsteroid {
                _input: typedefs::StopMiningAsteroidInput { key_index: 0 },
            };
            let stop_mining_asteroid_ix = Instruction::new_with_bytes(
                sage_program.id(),
                &instr.data(),
                vec![
                    AccountMeta::new(sage_program.payer(), true),
                    AccountMeta::new_readonly(player_profile, false),
                    AccountMeta::new_readonly(profile_faction, false),
                    AccountMeta::new(*fleet_id, false),
                    AccountMeta::new_readonly(*game_id, false),
                    AccountMeta::new_readonly(*game_state, false),
                    AccountMeta::new_readonly(*mine_item, false),
                    AccountMeta::new(*resource, false),
                    AccountMeta::new(*planet, false),
                    AccountMeta::new(*fuel_tank, false),
                    AccountMeta::new_readonly(fuel_cargo_type, false),
                    AccountMeta::new_readonly(*cargo_stats_definition, false),
                    AccountMeta::new(ata_fleet_fuel, false),
                    AccountMeta::new(*fuel_mint, false), // good
                    AccountMeta::new(pilot_user_points, false),
                    AccountMeta::new_readonly(pilot_points_category_pubkey, false),
                    AccountMeta::new_readonly(pilot_points_modifer_pubkey, false),
                    AccountMeta::new(mining_user_points, false),
                    AccountMeta::new_readonly(mining_points_category_pubkey, false),
                    AccountMeta::new_readonly(mining_points_modifier_pubkey, false),
                    AccountMeta::new(council_rank_user_points, false),
                    AccountMeta::new(council_rank_points_category_pubkey, false),
                    AccountMeta::new_readonly(council_rank_points_modifier_pubkey, false),
                    AccountMeta::new_readonly(progress_config, false),
                    AccountMeta::new_readonly(Points::id(), false),
                    AccountMeta::new_readonly(Cargo::id(), false),
                    AccountMeta::new_readonly(spl_token::id(), false),
                ],
            );

            let builder = sage_program
                .request()
                .instruction(ata_resource_to_ix)
                // .instruction(ata_ammo_from_ix)
                // .instruction(ata_food_from_ix)
                .instruction(fleet_state_handler_ix);

            let ix = builder.instructions()?;
            ixs.push(ix);

            let builder = sage_program.request().instruction(stop_mining_asteroid_ix);

            let ix = builder.instructions()?;
            ixs.push(ix);
        }
        _ => {}
    }

    Ok(ixs)
}
