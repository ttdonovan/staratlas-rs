use super::*;

pub(crate) fn refresh_fleet_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
) -> anyhow::Result<SageResponse> {
    log::info!("SageLabs > Refresh Fleet ({}): {}", fleet_id, sage.game_id);

    let (fleet, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;

    let fuel = sage.get_token_account_balances_by_owner(&fleet.fuel_tank)?;
    let ammo = sage.get_token_account_balances_by_owner(&fleet.ammo_bank)?;
    let cargo = sage.get_token_account_balances_by_owner(&fleet.cargo_hold)?;

    let mut mine_resource = None;
    let mut mine_mine_item = None;

    match &fleet_state {
        sage::FleetState::Idle(idle) => {
            let planets = sage.planet_accts(idle.sector)?;

            if let Some((planet_pubkey, _planet)) =
                planets.iter().find(|(_, planet)| planet.num_resources == 1)
            {
                let mut resources = sage.resource_accts(planet_pubkey)?;
                let (resource_id, resource) = resources.remove(0);
                let (mine_item_id, mine_item) = sage.mine_item_acct(&resource.mine_item)?;

                mine_resource = Some((resource_id, resource));
                mine_mine_item = Some((mine_item_id, mine_item));
            }
        }
        sage::FleetState::MineAsteroid(mine_asteroid) => {
            let mut resources = sage.resource_accts(&mine_asteroid.asteroid)?;
            let (resource_id, resource) = resources.remove(0);
            let (mine_item_id, mine_item) = sage.mine_item_acct(&resource.mine_item)?;

            mine_resource = Some((resource_id, resource));
            mine_mine_item = Some((mine_item_id, mine_item));
        }
        sage::FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
            let (_, starbase) = sage.starbase_acct(&starbase_loading_bay.starbase)?;
            let planets = sage.planet_accts(starbase.sector)?;
            if let Some((planet_pubkey, _planet)) =
                planets.iter().find(|(_, planet)| planet.num_resources == 1)
            {
                let mut resources = sage.resource_accts(planet_pubkey)?;
                let (resource_id, resource) = resources.remove(0);
                let (mine_item_id, mine_item) = sage.mine_item_acct(&resource.mine_item)?;

                mine_resource = Some((resource_id, resource));
                mine_mine_item = Some((mine_item_id, mine_item));
            }
        }
        _ => {
            unimplemented!();
        }
    }

    Ok(SageResponse::RefreshFleet(
        *fleet_id,
        (
            fleet,
            fleet_state,
            (fuel, ammo, cargo),
            mine_resource,
            mine_mine_item,
        ),
    ))
}

pub(crate) fn start_mining_asteroid_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
) -> anyhow::Result<SageResponse> {
    log::info!(
        "SageLabs > Start Mining Asteroid ({}): {}",
        fleet_id,
        sage.game_id
    );

    let result = match sage.start_mining_asteroid(fleet_id, fleet, state) {
        Ok(signature) => {
            let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
            Ok((signature, fleet_state))
        }
        Err(err) => Err(err),
    };

    Ok(SageResponse::StartMiningAsteroid((*fleet_id, result)))
}

pub(crate) fn stop_mining_asteroid_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
) -> anyhow::Result<SageResponse> {
    log::info!(
        "SageLabs > Stop Mining Asteroid ({}): {}",
        fleet_id,
        sage.game_id
    );

    let result = match sage.stop_mining_asteroid(fleet_id, fleet, state) {
        Ok(signature) => {
            let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id).unwrap();
            Ok((signature, fleet_state))
        }
        Err(err) => Err(err),
    };

    Ok(SageResponse::StopMiningAsteroid((*fleet_id, result)))
}

pub(crate) fn dock_to_starbase_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
) -> anyhow::Result<SageResponse> {
    log::info!(
        "SageLabs > Dock to Starbase ({}): {}",
        fleet_id,
        sage.game_id
    );

    let result = match sage.dock_to_starbase(fleet_id, fleet, state) {
        Ok(signature) => {
            let (fleet, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
            let fuel = sage.get_token_account_balances_by_owner(&fleet.fuel_tank)?;
            let ammo = sage.get_token_account_balances_by_owner(&fleet.ammo_bank)?;
            let cargo = sage.get_token_account_balances_by_owner(&fleet.cargo_hold)?;

            Ok((signature, fleet_state, (fuel, ammo, cargo)))
        }
        Err(err) => Err(err),
    };

    Ok(SageResponse::DockToStarbase((*fleet_id, result)))
}

pub(crate) fn undock_from_starbase_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
) -> anyhow::Result<SageResponse> {
    log::info!(
        "SageLabs > Undock from Starbase ({}): {}",
        &fleet_id,
        &sage.game_id
    );

    let result = match sage.undock_from_starbase(&fleet_id, &fleet, &state) {
        Ok(signature) => {
            let (fleet, fleet_state) = sage.fleet_with_state_accts(&fleet_id)?;

            let fuel = sage.get_token_account_balances_by_owner(&fleet.fuel_tank)?;
            let ammo = sage.get_token_account_balances_by_owner(&fleet.ammo_bank)?;
            let cargo = sage.get_token_account_balances_by_owner(&fleet.cargo_hold)?;

            Ok((signature, fleet_state, (fuel, ammo, cargo)))
        }
        Err(err) => Err(err),
    };

    Ok(SageResponse::UndockFromStarbase((*fleet_id, result)))
}

pub(crate) fn starbase_hangar_cargo_withdraw_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    starbase_id: &Pubkey,
    mint: &Pubkey,
) -> anyhow::Result<SageResponse> {
    log::info!(
        "SageLabs > Starbase Hangar Cargo Withdraw ({}): {}",
        fleet_id,
        sage.game_id
    );

    let res = sage.withdraw_from_fleet(
        fleet_id,
        fleet,
        starbase_id,
        mint,
        None, // withdraw all (max)
    );

    Ok(SageResponse::StarbaseHangarCargoWithdraw((*fleet_id, res)))
}

pub(crate) fn starbase_hangar_cargo_deposit_to_fleet_response(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    starbase_id: &Pubkey,
    cargo_pod_to: &Pubkey,
    cargo_deposit: bots::CargoDeposit,
    amount: u64,
) -> anyhow::Result<SageResponse> {
    log::info!(
        "SageLabs > Starbase Hangar Cargo Withdraw ({}): {}",
        fleet_id,
        sage.game_id
    );

    let mint = match cargo_deposit {
        bots::CargoDeposit::Fuel => sage.game_acct.mints.fuel,
        bots::CargoDeposit::Ammo => sage.game_acct.mints.ammo,
        bots::CargoDeposit::Food => sage.game_acct.mints.food,
    };

    let result = sage.deposit_to_fleet(
        &fleet_id,
        &fleet,
        &starbase_id,
        &cargo_pod_to,
        &mint,
        amount,
    );

    Ok(SageResponse::StarbaseHangarDepositToFleet((
        (*fleet_id, cargo_deposit),
        result,
    )))
}
