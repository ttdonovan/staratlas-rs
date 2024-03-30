use super::*;

pub fn refresh_fleet(
    fleet_id: Pubkey,
    data: (
        sage::Fleet,
        sage::FleetState,
        (u32, u32, u32),
        Option<(Pubkey, sage::Resource)>,
        Option<(Pubkey, sage::MineItem)>,
    ),
) -> anyhow::Result<MiningBot> {
    let (fleet, fleet_state, (fuel, ammo, cargo), resource, mine_item) = data;
    let resource = resource.unwrap();
    let mine_item = mine_item.unwrap();

    let mine_asteroid_emission_rate =
        calc_asteroid_mining_emission_rate(&fleet, &resource.1, &mine_item.1);

    let mine_asteroid_start = match &fleet_state {
        sage::FleetState::MineAsteroid(mine_asteroid) => mine_asteroid.start,
        _ => 0,
    };

    let mine_asteroid_amount = calc_asteroid_mining_amount(
        cargo,
        fleet.stats.cargo_stats.cargo_capacity,
        mine_asteroid_emission_rate,
        mine_asteroid_start,
    );

    let mine_asteroid_duraiton =
        calc_asteroid_mining_duration(mine_asteroid_amount, mine_asteroid_emission_rate);

    let mut autoplay = Autoplay::IsIdle;
    let mut mining_timer = time::Timer::default();

    match &fleet_state {
        sage::FleetState::MineAsteroid(_) => {
            autoplay = Autoplay::IsMiningAsteroid;
            mining_timer.set_duration(mine_asteroid_duraiton);
        }
        sage::FleetState::StarbaseLoadingBay(_) => {
            autoplay = Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Fuel);
        }
        _ => {}
    }

    let masked_fleet_id = ui_masked_pubkey(&fleet_id);
    let fleet_name = fleet.fleet_label().to_string();
    let mine_item_name = mine_item.1.name().to_string();

    let bot = MiningBot {
        fleet_id,
        masked_fleet_id,
        fleet_name,
        fleet,
        fleet_state,
        fuel_tank: fleet.fuel_tank,
        ammo_bank: fleet.ammo_bank,
        cargo_hold: fleet.cargo_hold,
        fuel_tank_amount: fuel,
        fuel_tank_capacity: fleet.stats.cargo_stats.fuel_capacity,
        ammo_bank_amount: ammo,
        ammo_bank_capacity: fleet.stats.cargo_stats.ammo_capacity,
        cargo_hold_amount: cargo,
        cargo_hold_capacity: fleet.stats.cargo_stats.cargo_capacity,
        resource: resource,
        mine_item: mine_item,
        mine_item_name,
        mine_asteroid_emission_rate,
        mine_asteroid_amount,
        mine_asteroid_duraiton,
        mining_timer,
        autoplay,
        txs: None,
        txs_counter: 0,
        txs_errors: 0,
        is_tx: false,
    };

    Ok(bot)
}

pub fn start_mining_asteroid_result(
    bot: &mut MiningBot,
    result: Result<(Signature, sage::FleetState), anyhow::Error>,
) {
    match result {
        Ok((signature, state)) => {
            bot.autoplay = Autoplay::IsMiningAsteroid;
            bot.set_fleet_state(state);
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            bot.is_tx = false;

            // start our mining timer
            bot.reset_mining_timer();
        }
        Err(err) => {
            log::error!(
                "[{}] Start Mining Asteroid: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            bot.is_tx = false;
        }
    }
}

pub fn stop_mining_asteroid_result(
    bot: &mut MiningBot,
    result: Result<(Signature, sage::FleetState), anyhow::Error>,
) {
    match result {
        Ok((signature, state)) => {
            bot.autoplay = Autoplay::StarbaseDock;
            bot.set_fleet_state(state);
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            bot.is_tx = false;
        }
        Err(err) => {
            log::error!(
                "[{}] Stop Mining Asteroid: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            bot.is_tx = false;
        }
    }
}

pub fn dock_to_starbase_result(
    bot: &mut MiningBot,
    result: Result<(Signature, sage::FleetState, (u32, u32, u32)), anyhow::Error>,
) {
    match result {
        Ok((signature, state, (fuel, ammo, cargo))) => {
            bot.autoplay = Autoplay::StarbaseHangarCargoWithdraw;

            bot.set_fleet_state(state);
            bot.set_fuel_amount(fuel);
            bot.set_ammo_amount(ammo);
            bot.set_cargo_amount(cargo);

            bot.txs = Some(signature);
            bot.txs_counter += 1;
            bot.is_tx = false;
        }
        Err(err) => {
            log::error!("[{}] Dock to Starbase: {:?}", bot.masked_fleet_id(), err);
            bot.txs_errors += 1;
            bot.is_tx = false;
        }
    }
}

pub fn undock_from_starbase_result(
    bot: &mut MiningBot,
    result: Result<(Signature, sage::FleetState, (u32, u32, u32)), anyhow::Error>,
) {
    match result {
        Ok((signature, state, (fuel, ammo, cargo))) => {
            bot.autoplay = Autoplay::IsIdle;

            bot.set_fleet_state(state);
            bot.set_fuel_amount(fuel);
            bot.set_ammo_amount(ammo);
            bot.set_cargo_amount(cargo);

            bot.txs = Some(signature);
            bot.txs_counter += 1;
            bot.is_tx = false;
        }
        Err(err) => {
            log::error!(
                "[{}] Undock from Starbase: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            bot.is_tx = false;
        }
    }
}

pub fn starbase_hangar_cargo_withdraw_result(
    bot: &mut MiningBot,
    result: Result<Option<Signature>, anyhow::Error>,
) {
    match result {
        Ok(signature) => {
            bot.autoplay = Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Fuel);
            bot.txs = signature;
            bot.txs_counter += 1;
            bot.is_tx = false;
        }
        Err(err) => {
            log::error!(
                "[{}] Widthdraw from Fleet: {:?}",
                bot.masked_fleet_id(),
                err
            );
            bot.txs_errors += 1;
            bot.is_tx = false;
        }
    }
}

pub fn starbase_hangar_deposit_to_fleet_result(
    bot: &mut MiningBot,
    cargo_deposit: CargoDeposit,
    result: Result<Signature, anyhow::Error>,
) {
    match result {
        Ok(signature) => {
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            bot.is_tx = false;

            bot.autoplay = match &cargo_deposit {
                CargoDeposit::Fuel => Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Ammo),
                CargoDeposit::Ammo => Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Food),
                _ => Autoplay::StarbaseUndock,
            };
        }
        Err(err) => {
            log::error!("[{}] Deposit to Fleet: {:?}", bot.masked_fleet_id(), err);
            bot.txs_errors += 1;
            bot.is_tx = false;
        }
    }
}
