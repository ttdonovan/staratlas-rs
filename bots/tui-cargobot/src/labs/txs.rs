use crate::{sage, time, Pubkey, MAX_CARGO_AMOUNT};
use sage::staratlas_sage_sdk::programs::staratlas_sage::typedefs;

pub fn is_cargo_hold_at_capacity(
    sage: &sage::SageContext,
    fleet: &sage::Fleet,
) -> anyhow::Result<bool> {
    let cargo_hold = &fleet.0.cargo_hold;
    let amount = sage.get_token_account_balances_by_owner(cargo_hold)?;

    Ok(amount as u64 >= MAX_CARGO_AMOUNT)
}

pub fn dock_to_starbase(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
    idle: &typedefs::Idle,
) -> anyhow::Result<()> {
    log::info!("[Sage Labs] - Prepare dock to starbase {:?}", idle.sector);
    let signature = sage.dock_to_starbase(&fleet_id, &fleet, &state)?;
    log::info!("[Sage Labs] - Dock to starbase: {:?}", signature);

    Ok(())
}

pub fn undock_from_starbase(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
) -> anyhow::Result<()> {
    log::info!("[Sage Labs] - Prepare to undock from starbase");
    let signature = sage.undock_from_starbase(&fleet_id, &fleet, &state)?;
    log::info!("[Sage Labs] - Undock from starbase: {:?}", signature);

    Ok(())
}

pub fn warp_to_coordinate(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    sector: [i64; 2],
) -> anyhow::Result<()> {
    log::info!("[Sage Labs] - Prepare to warp to {:?}", &sector);
    let signature = sage.warp_to_coordinate(&fleet_id, &fleet, sector.clone())?;
    log::info!("[Sage Labs] - Warping to {:?}: {:?}", &sector, signature);

    Ok(())
}

pub fn ready_to_exit_warp(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    move_warp: &typedefs::MoveWarp,
) -> anyhow::Result<bool> {
    let now = time::get_time() as i64;
    let warp_time = move_warp.warp_finish - move_warp.warp_start;
    let mut is_exit = false;

    log::info!(
        "[Sage Labs] - Warp time to {:?}: {:?}s",
        move_warp.to_sector,
        warp_time
    );

    if now > move_warp.warp_finish {
        log::info!("[Sage Labs] - Prepare ready to exit warp");
        let signature = sage.ready_to_exit_warp(&fleet_id, &fleet)?;
        log::info!("[Sage Labs] - Exit warp: {:?}", signature);
        is_exit = true;
    }

    Ok(is_exit)
}

pub fn withdraw_from_fleet_cargo_hold(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &&sage::Fleet,
    starbase_id: &Pubkey,
    mint: &Pubkey,
) -> anyhow::Result<()> {
    let cargo_mint = mint;

    log::info!("[Sage Labs] - Prepare to withdraw from fleet cargo hold");
    let signature = sage.withdraw_from_fleet(
        fleet_id,
        fleet,
        starbase_id,
        cargo_mint,
        None, // MAX_CARGO_AMOUNT
    )?;
    log::info!(
        "[Sage Labs] - Withdraw from fleet cargo hold: {:?}",
        signature
    );

    Ok(())
}

pub fn deposit_to_fleet_cargo_hold(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    starbase_id: &Pubkey,
    mint: &Pubkey,
) -> anyhow::Result<()> {
    // let (pubkey, cargo_pod_acct) = sage.starbase_cargo_pod_acct(starbase_id, fleet)?;
    // dbg!(pubkey, cargo_pod_acct);

    // let accounts = sage.get_token_accounts_by_owner(&pubkey)?;

    // for acct in accounts {
    //     dbg!(acct);
    // }

    let cargo_hold = &fleet.0.cargo_hold;
    let cargo_mint = mint;

    log::info!("[Sage Labs] - Prepare to deposit to fleet cargo hold");
    let signature = sage.deposit_to_fleet(
        fleet_id,
        fleet,
        starbase_id,
        cargo_hold,
        cargo_mint,
        MAX_CARGO_AMOUNT,
    )?;
    log::info!("[Sage Labs] - Deposit to fleet cargo hold: {:?}", signature);

    Ok(())
}

pub fn deposit_to_fleet_fuel_tank(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    starbase_id: &Pubkey,
) -> anyhow::Result<()> {
    let fuel_tank = &fleet.0.fuel_tank;
    let cargo_stats = &fleet.0.stats.cargo_stats;
    let fuel_capacity = cargo_stats.fuel_capacity;
    let fuel_amount = sage.get_token_account_balances_by_owner(fuel_tank)?;

    if (fuel_amount as f32 / fuel_capacity as f32) < 0.5 {
        let fuel_mint = &sage.game_acct.0.mints.fuel;
        let amount = (fuel_capacity - fuel_amount) as u64;

        log::info!("[Sage Labs] - Prepare to refuel fleet");
        let signature =
            sage.deposit_to_fleet(fleet_id, fleet, starbase_id, fuel_tank, fuel_mint, amount)?;
        log::info!("[Sage Labs] - Refuel fleet: {:?}", signature);
    }

    Ok(())
}
