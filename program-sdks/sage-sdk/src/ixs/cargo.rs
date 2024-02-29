use super::*;

use staratlas_sage::{instruction, state, typedefs};

use crate::{derive, find, Fleet, Game};

pub fn deposit_to_fleet<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    cargo_program: &Program<C>,
    fleet: (&Pubkey, &Fleet),
    game: (&Pubkey, &Game),
    starbase: &Pubkey,
    cargo_pod_to: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let (fleet_id, fleet_acct) = fleet;
    let (game_id, game_acct) = game;

    let game_state = &game_acct.0.game_state;

    let player_profile = &fleet_acct.0.owner_profile;
    let (profile_faction, _) = find_profile_faction_address(&player_profile)?;

    let starbase_acct = derive_account::<_, state::Starbase>(sage_program, &starbase)?;

    let (sage_player_profile, _) = find::sage_player_profile_address(game_id, &player_profile);
    let (starbase_player, _) =
        find::starbase_player_address(&starbase, &sage_player_profile, starbase_acct.seq_id);

    let cargo_pods = derive::cargo_pod_accounts(cargo_program, &starbase_player)?;
    let (cargo_pod, cargo_pod_acct) = cargo_pods
        .first()
        .expect("at least one cargo pod for starbase player");

    let cargo_pod_from = cargo_pod;

    let ata_token_from = get_associated_token_address(cargo_pod_from, mint);
    let ata_token_to = get_associated_token_address(&cargo_pod_to, mint);
    let ata_token_to_ix = create_associated_token_account_idempotent(
        &sage_program.payer(),
        &ata_token_to,
        mint,
        &spl_token::id(),
    );

    let cargo_stats_def = &cargo_pod_acct.0.stats_definition;
    let seq_id = cargo_pod_acct.0.seq_id;
    let (mint_cargo_type, _) = find::cargo_type_address(cargo_stats_def, mint, seq_id);

    let instr = instruction::DepositCargoToFleet {
        _input: typedefs::DepositCargoToFleetInput {
            amount,
            key_index: 0,
        },
    };
    let deposit_cargo_to_fleet_ix = Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(*player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(*game_state, false),
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(*starbase, false),
            AccountMeta::new_readonly(starbase_player, false),
            AccountMeta::new(*cargo_pod_from, false),
            AccountMeta::new(*cargo_pod_to, false),
            AccountMeta::new_readonly(mint_cargo_type, false),
            AccountMeta::new_readonly(*cargo_stats_def, false),
            AccountMeta::new(ata_token_from, false),
            AccountMeta::new(ata_token_to, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(cargo_program.id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(*starbase, false),
        ],
    );

    let builder = sage_program
        .request()
        .instruction(ata_token_to_ix)
        .instruction(deposit_cargo_to_fleet_ix);

    let ixs = builder.instructions()?;
    Ok(ixs)
}

pub fn withdraw_from_fleet<C: Deref<Target = impl Signer> + Clone>(
    sage_program: &Program<C>,
    cargo_program: &Program<C>,
    fleet: (&Pubkey, &Fleet),
    game: (&Pubkey, &Game),
    starbase: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) -> anyhow::Result<Vec<Instruction>> {
    let (fleet_id, fleet_acct) = fleet;
    let (game_id, game_acct) = game;

    let game_state = &game_acct.0.game_state;

    let player_profile = &fleet_acct.0.owner_profile;
    let (profile_faction, _) = find_profile_faction_address(&player_profile)?;

    let starbase_acct = derive_account::<_, state::Starbase>(sage_program, &starbase)?;

    let (sage_player_profile, _) = find::sage_player_profile_address(game_id, &player_profile);
    let (starbase_player, _) =
        find::starbase_player_address(&starbase, &sage_player_profile, starbase_acct.seq_id);

    let cargo_pods = derive::cargo_pod_accounts(cargo_program, &starbase_player)?;
    let (cargo_pod, cargo_pod_acct) = cargo_pods
        .first()
        .expect("at least one cargo pod for starbase player");

    let cargo_pod_from = &fleet_acct.0.cargo_hold;
    let cargo_pod_to = cargo_pod;

    let ata_token_from = get_associated_token_address(cargo_pod_from, mint);
    let ata_token_to = get_associated_token_address(&cargo_pod_to, mint);
    let ata_token_to_ix = create_associated_token_account_idempotent(
        &sage_program.payer(),
        &ata_token_to,
        mint,
        &spl_token::id(),
    );

    let cargo_stats_def = &cargo_pod_acct.0.stats_definition;
    let seq_id = cargo_pod_acct.0.seq_id;
    let (mint_cargo_type, _) = find::cargo_type_address(cargo_stats_def, mint, seq_id);

    let instr = instruction::WithdrawCargoFromFleet {
        _input: typedefs::WithdrawCargoFromFleetInput {
            amount,
            key_index: 0,
        },
    };
    let withdraw_cargo_from_fleet_ix = Instruction::new_with_bytes(
        sage_program.id(),
        &instr.data(),
        vec![
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new_readonly(*player_profile, false),
            AccountMeta::new_readonly(profile_faction, false),
            AccountMeta::new(*fleet_id, false),
            AccountMeta::new_readonly(*game_id, false),
            AccountMeta::new_readonly(*game_state, false),
            AccountMeta::new_readonly(*starbase, false),
            AccountMeta::new_readonly(starbase_player, false),
            AccountMeta::new(*cargo_pod_from, false),
            AccountMeta::new(*cargo_pod_to, false),
            AccountMeta::new_readonly(mint_cargo_type, false),
            AccountMeta::new_readonly(*cargo_stats_def, false),
            AccountMeta::new(ata_token_from, false),
            AccountMeta::new(ata_token_to, false),
            AccountMeta::new(*mint, false),
            AccountMeta::new(sage_program.payer(), true),
            AccountMeta::new(cargo_program.id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(*starbase, false),
        ],
    );

    let builder = sage_program
        .request()
        .instruction(ata_token_to_ix)
        .instruction(withdraw_cargo_from_fleet_ix);

    let ixs = builder.instructions()?;
    Ok(ixs)
}
