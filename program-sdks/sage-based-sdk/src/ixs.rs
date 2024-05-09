use anchor_client::{
    anchor_lang::{
        prelude::{AccountMeta, Pubkey},
        InstructionData,
    },
    solana_sdk::{instruction::Instruction, signature::Signer},
    Program,
};
use spl_associated_token_account::get_associated_token_address;

use staratlas_cargo::ID as CARGO_ID;
use staratlas_points::ID as POINTS_ID;
use staratlas_sage::{instruction, typedefs};

use std::ops::Deref;

use crate::{addr, CargoPod, Fleet, Game};

pub fn cargo_deposit_to_fleet<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    cargo_pod: (&Pubkey, &CargoPod),
    starbase: &Pubkey,
    cargo_pod_to: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (cargo_pod_id, cargo_pod) = cargo_pod;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);
    let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);

    let starbase_seq_id = 0; // TODO: this should come from the starbase account
    let (starbase_player, _) =
        addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

    let (mint_cargo_type, _) =
        addr::cargo_type_address(&cargo_pod.stats_definition, mint, cargo_pod.seq_id);

    let ata_token_from = get_associated_token_address(cargo_pod_id, mint);
    let ata_token_to = get_associated_token_address(cargo_pod_to, mint);

    let instr = instruction::DepositCargoToFleet {
        _input: typedefs::DepositCargoToFleetInput {
            amount,
            key_index: 0,
        },
    };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(*starbase, false),
            AccountMeta::new_readonly(starbase_player, false),
            AccountMeta::new(*cargo_pod_id, false),
            AccountMeta::new(*cargo_pod_to, false),
            AccountMeta::new_readonly(mint_cargo_type, false),
            AccountMeta::new_readonly(cargo_pod.stats_definition, false),
            AccountMeta::new(ata_token_from, false),
            AccountMeta::new(ata_token_to, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(CARGO_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(*starbase, false),
        ],
    )
}

pub fn cargo_withdraw_from_fleet<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    cargo_pod: (&Pubkey, &CargoPod),
    starbase: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (cargo_pod_id, cargo_pod) = cargo_pod;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);
    let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);

    let starbase_seq_id = 0; // TODO: this should come from the starbase account
    let (starbase_player, _) =
        addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

    let (mint_cargo_type, _) =
        addr::cargo_type_address(&cargo_pod.stats_definition, mint, cargo_pod.seq_id);

    let ata_token_from = get_associated_token_address(&fleet.cargo_hold, mint);
    let ata_token_to = get_associated_token_address(&cargo_pod_id, mint);

    let instr = instruction::WithdrawCargoFromFleet {
        _input: typedefs::WithdrawCargoFromFleetInput {
            amount,
            key_index: 0,
        },
    };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new_readonly(*starbase, false),
            AccountMeta::new_readonly(starbase_player, false),
            AccountMeta::new(fleet.cargo_hold, false),
            AccountMeta::new(*cargo_pod_id, false),
            AccountMeta::new_readonly(mint_cargo_type, false),
            AccountMeta::new_readonly(cargo_pod.stats_definition, false),
            AccountMeta::new(ata_token_from, false),
            AccountMeta::new(ata_token_to, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new(CARGO_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(*starbase, false),
        ],
    )
}

pub fn dock_to_starbase<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    sector: [i64; 2],
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);
    let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);

    let (starbase, _) = addr::starbase_address(game_id, sector);
    // let starbase_acct = program.account::<state::Starbase>(&starbase).await.unwrap();
    let starbase_seq_id = 0; // TODO: this should come from the starbase account

    let (starbase_player, _) =
        addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

    let instr = instruction::IdleToLoadingBay { _key_index: 0 };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new_readonly(starbase, false),
            AccountMeta::new(starbase_player, false),
        ],
    )
}

pub fn undock_from_starbase<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    starbase: &Pubkey,
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);
    let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);

    let starbase_seq_id = 0; // TODO: this should come from the starbase account
    let (starbase_player, _) =
        addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

    let instr = instruction::LoadingBayToIdle { _key_index: 0 };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new_readonly(*starbase, false),
            AccountMeta::new(starbase_player, false),
            AccountMeta::new(*starbase, false),
        ],
    )
}

pub fn fleet_state_handler<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    mine_item: &Pubkey,
    mine_item_mint: &Pubkey,
    resource: &Pubkey,
    planet: &Pubkey,
    sector: [i64; 2],
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;
    let cargo_stats_definition = game.cargo.stats_definition;

    let (fleet_id, fleet) = fleet;

    let (starbase, _) = addr::starbase_address(game_id, sector);

    // ata for resource
    let ata_resource_from = get_associated_token_address(&mine_item, mine_item_mint);
    let ata_resource_to = get_associated_token_address(&fleet.cargo_hold, mine_item_mint);

    // let ata_resource_to_ix = create_associated_token_account_idempotent(
    //     &sage_program.payer(),
    //     &ata_resource_to,
    //     mine_item_mint,
    //     &spl_token::id(),
    // );

    // ata for ammo, food and fuel
    let ata_fleet_ammo = get_associated_token_address(&fleet.ammo_bank, &game.mints.ammo);
    let ata_fleet_food = get_associated_token_address(&fleet.cargo_hold, &game.mints.food);

    // let cargo_stats_definition_acct = derive::cargo_stats_definition_account(cargo_program, cargo_stats_definition)?;
    // dbg!(&cargo_stats_definition_acct.0.seq_id);

    let seq_id = 0;
    let (food_cargo_type, _) =
        addr::cargo_type_address(&cargo_stats_definition, &game.mints.food, seq_id);
    let (ammo_cargo_type, _) =
        addr::cargo_type_address(&cargo_stats_definition, &game.mints.ammo, seq_id);
    let (resource_cargo_type, _) =
        addr::cargo_type_address(&cargo_stats_definition, mine_item_mint, seq_id);

    let instr = instruction::FleetStateHandler {};

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new(fleet.cargo_hold, false),
            AccountMeta::new(fleet.ammo_bank, false),
            AccountMeta::new_readonly(*mine_item, false),
            AccountMeta::new(*resource, false),
            AccountMeta::new(*planet, false),
            AccountMeta::new_readonly(starbase, false),
            AccountMeta::new(ata_fleet_food, false),
            AccountMeta::new(ata_fleet_ammo, false),
            AccountMeta::new(ata_resource_from, false),
            AccountMeta::new(ata_resource_to, false),
            AccountMeta::new(game.mints.food, false),
            AccountMeta::new(game.mints.ammo, false),
            AccountMeta::new_readonly(food_cargo_type, false),
            AccountMeta::new_readonly(ammo_cargo_type, false),
            AccountMeta::new_readonly(resource_cargo_type, false),
            AccountMeta::new_readonly(cargo_stats_definition, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(CARGO_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
    )
}

pub fn start_mining_asteroid<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    mine_item: &Pubkey,
    resource: &Pubkey,
    planet: &Pubkey,
    sector: [i64; 2],
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);
    let (sage_player_profile, _) = addr::sage_player_profile_address(&game_id, &player_profile);

    let ata_fleet_fuel = get_associated_token_address(&fleet.fuel_tank, &game.mints.fuel);

    let (starbase, _) = addr::starbase_address(game_id, sector);
    // let starbase_acct = program.account::<state::Starbase>(&starbase).await.unwrap();
    let starbase_seq_id = 0; // TODO: this should come from the starbase account

    let (starbase_player, _) =
        addr::starbase_player_address(&starbase, &sage_player_profile, starbase_seq_id);

    let instr = instruction::StartMiningAsteroid {
        _input: typedefs::KeyIndexInput { key_index: 0 },
    };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new(ata_fleet_fuel, false),
            AccountMeta::new_readonly(starbase, false),
            AccountMeta::new_readonly(starbase_player, false),
            AccountMeta::new_readonly(*mine_item, false),
            AccountMeta::new(*resource, false),
            AccountMeta::new(*planet, false),
        ],
    )
}

pub fn stop_mining_asteroid<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    mine_item: &Pubkey,
    resource: &Pubkey,
    planet: &Pubkey,
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;
    let cargo_stats_definition = game.cargo.stats_definition;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);

    let ata_fleet_fuel = get_associated_token_address(&fleet.fuel_tank, &game.mints.fuel);

    let seq_id = 0;
    let (fuel_cargo_type, _) =
        addr::cargo_type_address(&cargo_stats_definition, &game.mints.fuel, seq_id);

    let (pilot_user_points, _) =
        addr::user_points_account_address(&game.points.pilot_xp_category.category, &player_profile);
    let (mining_user_points, _) = addr::user_points_account_address(
        &game.points.mining_xp_category.category,
        &player_profile,
    );
    let (council_rank_user_points, _) = addr::user_points_account_address(
        &game.points.council_rank_xp_category.category,
        &player_profile,
    );

    let (progress_config, _) = addr::progression_config_address(game_id);

    let instr = instruction::StopMiningAsteroid {
        _input: typedefs::StopMiningAsteroidInput { key_index: 0 },
    };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new_readonly(*mine_item, false),
            AccountMeta::new(*resource, false),
            AccountMeta::new(*planet, false),
            AccountMeta::new(fleet.fuel_tank, false),
            AccountMeta::new_readonly(fuel_cargo_type, false),
            AccountMeta::new_readonly(cargo_stats_definition, false),
            AccountMeta::new(ata_fleet_fuel, false),
            AccountMeta::new(game.mints.fuel, false),
            AccountMeta::new(pilot_user_points, false),
            AccountMeta::new_readonly(game.points.pilot_xp_category.category, false),
            AccountMeta::new_readonly(game.points.pilot_xp_category.modifier, false),
            AccountMeta::new(mining_user_points, false),
            AccountMeta::new_readonly(game.points.mining_xp_category.category, false),
            AccountMeta::new_readonly(game.points.mining_xp_category.modifier, false),
            AccountMeta::new(council_rank_user_points, false),
            AccountMeta::new(game.points.council_rank_xp_category.category, false),
            AccountMeta::new_readonly(game.points.council_rank_xp_category.modifier, false),
            AccountMeta::new_readonly(progress_config, false),
            AccountMeta::new_readonly(POINTS_ID, false),
            AccountMeta::new_readonly(CARGO_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
    )
}

pub fn warp_to_coordinate<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
    coordinate: [i64; 2],
) -> Instruction {
    let (game_id, game) = game;
    let game_state_id = game.game_state;
    let cargo_stats_definition = game.cargo.stats_definition;

    let (fleet_id, fleet) = fleet;
    let player_profile = fleet.owner_profile;

    let (profile_faction, _) = addr::profile_faction_address(&player_profile);

    let ata_fleet_fuel = get_associated_token_address(&fleet.fuel_tank, &game.mints.fuel);

    let seq_id = 0;
    let (fuel_cargo_type, _) =
        addr::cargo_type_address(&cargo_stats_definition, &game.mints.fuel, seq_id);

    let instr = instruction::WarpToCoordinate {
        _input: typedefs::WarpToCoordinateInput {
            key_index: 0,
            to_sector: coordinate,
        },
    };

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(game_state_id, false),
            AccountMeta::new(fleet.fuel_tank, false),
            AccountMeta::new_readonly(fuel_cargo_type, false),
            AccountMeta::new_readonly(cargo_stats_definition, false),
            AccountMeta::new(ata_fleet_fuel, false),
            AccountMeta::new(game.mints.fuel, false),
            AccountMeta::new_readonly(CARGO_ID, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
    )
}

pub fn warp_ready_to_exit<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    game: (&Pubkey, &Game),
    fleet: (&Pubkey, &Fleet),
) -> Instruction {
    let (fleet_id, fleet_acct) = fleet;
    let (game_id, game) = game;

    let player_profile = fleet_acct.owner_profile;

    let (pilot_user_points, _) =
        addr::user_points_account_address(&game.points.pilot_xp_category.category, &player_profile);
    let (council_rank_user_points, _) = addr::user_points_account_address(
        &game.points.council_rank_xp_category.category,
        &player_profile,
    );

    let (progress_config, _) = addr::progression_config_address(game_id);

    let instr = instruction::FleetStateHandler {};

    Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new(pilot_user_points, false),
            AccountMeta::new_readonly(game.points.pilot_xp_category.category, false),
            AccountMeta::new_readonly(game.points.pilot_xp_category.modifier, false),
            AccountMeta::new(council_rank_user_points, false),
            AccountMeta::new(game.points.council_rank_xp_category.category, false),
            AccountMeta::new_readonly(game.points.council_rank_xp_category.modifier, false),
            AccountMeta::new_readonly(player_profile, false),
            AccountMeta::new_readonly(progress_config, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(POINTS_ID, false),
        ],
    )
}
