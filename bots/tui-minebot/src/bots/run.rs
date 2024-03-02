use super::*;

fn get_and_set_fleet_state(bot: &mut MiningBot, sage: &sage::SageContext) -> anyhow::Result<()> {
    let (_, fleet_state) = sage.fleet_with_state_accts(&bot.fleet.0)?;
    bot.fleet.4 = fleet_state;

    match &bot.fleet.4 {
        FleetState::MineAsteroid(mine_asteroid) => {
            bot.mine_asteroid_start = mine_asteroid.start;
        }
        _ => {
            bot.mine_asteroid_start = 0;
        }
    }

    bot.is_fleet_state_dirty = false;

    Ok(())
}

fn get_and_set_token_balances_for_fuel_ammo_and_cargo(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    get_and_set_token_balance_for_fuel(bot, sage)?;
    get_and_set_token_balance_for_ammo_bank(bot, sage)?;
    get_and_set_token_balance_for_cargo_hold(bot, sage)?;

    Ok(())
}

fn get_and_set_token_balance_for_fuel(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    let fuel_amount = sage.get_token_account_balances_by_owner(&bot.fuel_tank.0)?;
    bot.set_fuel_amount(fuel_amount);

    Ok(())
}

fn get_and_set_token_balance_for_ammo_bank(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    let ammo_amount = sage.get_token_account_balances_by_owner(&bot.ammo_bank.0)?;
    bot.set_ammo_amount(ammo_amount);

    Ok(())
}

fn get_and_set_token_balance_for_cargo_hold(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    let cargo_amount = sage.get_token_account_balances_by_owner(&bot.cargo_hold.0)?;
    bot.set_cargo_amount(cargo_amount);

    Ok(())
}

fn calc_and_set_mine_asteroid_amount(bot: &mut MiningBot) {
    bot.mine_asteroid_amount = calc_asteroid_mining_amount(
        bot.cargo_hold.1,
        bot.cargo_hold.2,
        bot.mine_asteroid_emission_rate,
        bot.mine_asteroid_start,
    );
}

fn calc_and_set_mine_asteroid_duration(bot: &mut MiningBot) {
    bot.mine_asteroid_duraiton =
        calc_asteroid_mining_duration(bot.mine_asteroid_amount, bot.mine_asteroid_emission_rate);
}

fn autoplay_is_idle_handler(bot: &mut MiningBot, sage: &sage::SageContext) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        let _ = get_and_set_token_balances_for_fuel_ammo_and_cargo(bot, sage)?;
        let (_, actual, capacity) = &bot.cargo_hold;

        if (*actual as f32 / *capacity as f32) <= 0.9 {
            bot.autoplay = Autoplay::StartMiningAsteroid;
        } else {
            bot.autoplay = Autoplay::StarbaseDock;
        }
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_start_mining_asteroid_handler(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        get_and_set_token_balances_for_fuel_ammo_and_cargo(bot, sage)?;
        calc_and_set_mine_asteroid_amount(bot);
        calc_and_set_mine_asteroid_duration(bot);

        if bot.mine_asteroid_duraiton != Duration::ZERO {
            if txs::sage_start_mining_asteroid(bot, sage).is_ok() {
                bot.autoplay = Autoplay::IsMiningAsteroid;

                // start our mining timer
                bot.mining_timer.set_duration(bot.mine_asteroid_duraiton);
                bot.mining_timer.reset();
            };
        } else {
            bot.autoplay = Autoplay::IsIdle;
        }
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_is_mining_asteroid_handler(bot: &mut MiningBot) -> anyhow::Result<()> {
    if bot.is_fleet_mining() {
        if bot.mining_timer.finished() {
            bot.autoplay = Autoplay::StopMiningAsteroid;
        }
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_stop_mining_asteroid_handler(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    if bot.is_fleet_mining() {
        if txs::sage_stop_mining_asteroid(bot, sage).is_ok() {
            bot.autoplay = Autoplay::IsIdle;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_starbase_dock_handler(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        if txs::sage_dock_to_starbase(bot, sage).is_ok() {
            bot.autoplay = Autoplay::StarbaseHangarCargoWithdraw;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_undo_starbase_dock_handler(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    if bot.is_fleet_at_starbase() {
        if txs::sage_undock_from_starbase(bot, sage).is_ok() {
            bot.autoplay = Autoplay::IsIdle;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

pub fn run_update(
    bot: &mut MiningBot,
    dt: Duration,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    bot.tick(dt);

    if bot.is_fleet_state_dirty {
        get_and_set_fleet_state(bot, sage)?;

        // given the state was dirty, we need to re-calculate the mining amount and duration and other token balances
        match &bot.autoplay {
            Autoplay::IsMiningAsteroid => {
                let _ = get_and_set_token_balances_for_fuel_ammo_and_cargo(bot, sage)?;
                calc_and_set_mine_asteroid_amount(bot);
                calc_and_set_mine_asteroid_duration(bot);

                bot.mining_timer.set_duration(bot.mine_asteroid_duraiton);
                bot.mining_timer.reset();
            }
            Autoplay::StarbaseHangerCargoDeposit(deposit) => match deposit {
                CargoDeposit::Fuel => {
                    get_and_set_token_balance_for_fuel(bot, sage)?;
                }
                CargoDeposit::Ammo => {
                    get_and_set_token_balance_for_ammo_bank(bot, sage)?;
                }
                CargoDeposit::Food => {
                    get_and_set_token_balance_for_cargo_hold(bot, sage)?;
                }
            },
            _ => {}
        }
    }

    match &bot.autoplay {
        Autoplay::IsIdle => autoplay_is_idle_handler(bot, sage)?,
        Autoplay::StartMiningAsteroid => autoplay_start_mining_asteroid_handler(bot, sage)?,
        Autoplay::IsMiningAsteroid => autoplay_is_mining_asteroid_handler(bot)?,
        Autoplay::StopMiningAsteroid => autoplay_stop_mining_asteroid_handler(bot, sage)?,
        Autoplay::StarbaseDock => autoplay_starbase_dock_handler(bot, sage)?,
        Autoplay::StarbaseUndock => autoplay_undo_starbase_dock_handler(bot, sage)?,
        Autoplay::StarbaseHangarCargoWithdraw => {
            if let Some(starbase) = bot.starbase_id() {
                let starbase = starbase.clone();
                if txs::sage_mine_item_widthdraw_from_fleet(bot, &starbase, sage).is_ok() {
                    bot.autoplay = Autoplay::StarbaseHangerCargoDeposit(CargoDeposit::Fuel);
                };
            } else {
                bot.is_fleet_state_dirty = true;
            }
        }
        Autoplay::StarbaseHangerCargoDeposit(deposit) => {
            if let Some(starbase) = bot.starbase_id() {
                let starbase = starbase.clone();

                match deposit {
                    CargoDeposit::Fuel => {
                        // Fuel tank refuel
                        let (fuel_tank, actual, capacity) = &bot.fuel_tank.clone();
                        let fuel_mint = &sage.game_acct.0.mints.fuel;
                        let amount = (capacity - actual) as u64;

                        if (*actual as f32 / *capacity as f32) < 0.5 {
                            if txs::sage_deposit_to_fleet(
                                bot, &starbase, fuel_tank, fuel_mint, amount, sage,
                            )
                            .is_ok()
                            {
                                bot.autoplay =
                                    Autoplay::StarbaseHangerCargoDeposit(CargoDeposit::Ammo);
                            };
                        } else {
                            bot.autoplay = Autoplay::StarbaseHangerCargoDeposit(CargoDeposit::Ammo);
                        }
                    }
                    CargoDeposit::Ammo => {
                        // Ammo bank rearm
                        let (ammo_bank, actual, capacity) = &bot.ammo_bank.clone();
                        let ammo_mint = &sage.game_acct.0.mints.ammo;
                        let amount = (capacity - actual) as u64;

                        if (*actual as f32 / *capacity as f32) < 0.5 {
                            if txs::sage_deposit_to_fleet(
                                bot, &starbase, ammo_bank, ammo_mint, amount, sage,
                            )
                            .is_ok()
                            {
                                bot.autoplay =
                                    Autoplay::StarbaseHangerCargoDeposit(CargoDeposit::Food);
                            };
                        } else {
                            bot.autoplay = Autoplay::StarbaseHangerCargoDeposit(CargoDeposit::Food);
                        }
                    }
                    CargoDeposit::Food => {
                        // Cargo hold supply (food)
                        let (cargo_hold, actual, capacity) = &bot.cargo_hold.clone();
                        let food_mint = &sage.game_acct.0.mints.food;
                        let amount = (*capacity as f32 * 0.05) as u64;

                        if (*actual as f32 / *capacity as f32) < 0.05 {
                            if txs::sage_deposit_to_fleet(
                                bot, &starbase, cargo_hold, food_mint, amount, sage,
                            )
                            .is_ok()
                            {
                                bot.autoplay = Autoplay::StarbaseUndock;
                            };
                        } else {
                            bot.autoplay = Autoplay::StarbaseUndock;
                        }
                    }
                };
            } else {
                bot.is_fleet_state_dirty = true;
            }
        }
    }

    Ok(())
}
