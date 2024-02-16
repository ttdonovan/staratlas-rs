use super::*;

use staratlas_cargo::program::Cargo;
use staratlas_sage::{instruction, state, typedefs};

use crate::{derive, find, Fleet, FleetState, Game};

pub fn start_mining_asteroid<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, (&Fleet, &FleetState)),
    game: (&Pubkey, &Game),
) -> anyhow::Result<Vec<Instruction>> {
    let (fleet_pubkey, (fleet, fleet_state)) = fleet;
    let (game_pubkey, game) = game;

    match fleet_state {
        FleetState::Idle(idle) => {
            let fleet_id = fleet_pubkey;
            let player_profile = fleet.0.owner_profile;
            let (profile_faction, _) = find_profile_faction_address(&player_profile)?;
            let game_id = game_pubkey;
            let game_state = &game.0.game_state;

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
                .find(|(_, planet)| planet.0.num_resources == 1)
                .expect("planet with resources");

            let resources = derive::resource_accounts(sage_program, game_id, &planet)?;
            let (resource, resource_acct) = resources.first().expect("at least one resource");
            let mine_item = &resource_acct.0.mine_item;

            let instr = instruction::StartMiningAsteroid {
                _input: typedefs::StartMiningAsteroidInput { key_index: 0 },
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
                    AccountMeta::new_readonly(starbase, false),
                    AccountMeta::new_readonly(starbase_player, false),
                    AccountMeta::new_readonly(*mine_item, false),
                    AccountMeta::new(*resource, false),
                    AccountMeta::new(planet, false),
                ],
            );

            let builder = sage_program.request().instruction(start_mining_asteroid_ix);

            let ixs = builder.instructions()?;
            Ok(ixs)
        }
        _ => Ok(vec![]),
    }
}

pub fn stop_mining_asteroid<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    fleet: (&Pubkey, (&Fleet, &FleetState)),
    game: (&Pubkey, &Game),
) -> anyhow::Result<Vec<Instruction>> {
    let (fleet_pubkey, (fleet, fleet_state)) = fleet;
    let (game_pubkey, game) = game;

    match fleet_state {
        FleetState::MineAsteroid(mine_asteroid) => {
            let fleet_id = fleet_pubkey;
            let player_profile = fleet.0.owner_profile;
            let game_id = game_pubkey;
            let game_state = &game.0.game_state;

            // game mints
            let ammo_mint = &game.0.mints.ammo;
            let food_mint = &game.0.mints.food;
            let fuel_mint = &game.0.mints.fuel;

            // cargo stats definition
            let cargo_stats_definition = &game.0.cargo.stats_definition;
            // let cargo_stats_definition_acct = derive::cargo_stats_definition_account(cargo_program, cargo_stats_definition)?;
            // dbg!(&cargo_stats_definition_acct.0.seq_id);
            let seq_id = 1;

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
            let cargo_hold = &fleet.0.cargo_hold;
            let ammo_bank = &fleet.0.ammo_bank;
            let fuel_tank = &fleet.0.fuel_tank;

            let ata_resource_from = get_associated_token_address(&mine_item, resource_mint);
            let ata_resource_to = get_associated_token_address(cargo_hold, resource_mint);

            let ata_resource_to_ix = create_associated_token_account_idempotent(
                &sage_program.payer(),
                &ata_resource_to,
                resource_mint,
                &spl_token::id(),
            );

            let ata_fleet_ammo = get_associated_token_address(ammo_bank, ammo_mint);
            let ata_fleet_food = get_associated_token_address(cargo_hold, food_mint);
            let ata_fleet_fuel = get_associated_token_address(fuel_tank, fuel_mint);

            let food_token_from = &ata_fleet_food;
            let ammo_token_from = &ata_fleet_ammo;

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

            let instr = instruction::FleetStateHandler {};
            let fleet_state_handler_ix = Instruction::new_with_bytes(
                sage_program.id(),
                &instr.data(),
                vec![
                    AccountMeta::new(*fleet_id, false),
                    AccountMeta::new_readonly(profile_faction, false),
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
                    AccountMeta::new(*resource, false),
                    AccountMeta::new(*planet, false),
                    AccountMeta::new(*fuel_tank, false),
                    AccountMeta::new_readonly(fuel_cargo_type, false),
                    AccountMeta::new_readonly(*cargo_stats_definition, false),
                    AccountMeta::new(ata_fleet_fuel, false),
                    AccountMeta::new(*fuel_mint, false),
                    AccountMeta::new_readonly(Cargo::id(), false),
                    AccountMeta::new_readonly(spl_token::id(), false),
                ],
            );

            let builder = sage_program
                .request()
                .instruction(ata_resource_to_ix)
                .instruction(fleet_state_handler_ix)
                .instruction(stop_mining_asteroid_ix);

            let ixs = builder.instructions()?;
            Ok(ixs)
        }
        _ => Ok(vec![]),
    }
}
