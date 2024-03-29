use std::sync::mpsc::Sender;

use super::*;

use crate::labs;
use labs::Event as TxEvent;

fn get_and_set_fleet_state(bot: &mut MiningBot, sage: &sage::SageContext) -> anyhow::Result<()> {
    let (_, fleet_state) = sage.fleet_with_state_accts(&bot.fleet.0)?;
    bot.fleet.4 = fleet_state;

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
    log::info!(
        "[{}] Token balance Fuel Tank: {:?}",
        &bot.masked_fleet_id(),
        fuel_amount
    );

    bot.set_fuel_amount(fuel_amount);
    Ok(())
}

fn get_and_set_token_balance_for_ammo_bank(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    let ammo_amount = sage.get_token_account_balances_by_owner(&bot.ammo_bank.0)?;
    log::info!(
        "[{}] Token balance Ammo Bank: {:?}",
        &bot.masked_fleet_id(),
        ammo_amount
    );

    bot.set_ammo_amount(ammo_amount);
    Ok(())
}

fn get_and_set_token_balance_for_cargo_hold(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    let cargo_amount = sage.get_token_account_balances_by_owner(&bot.cargo_hold.0)?;
    log::info!(
        "[{}] Token balance Cargo Hold: {:?}",
        &bot.masked_fleet_id(),
        cargo_amount
    );

    bot.set_cargo_amount(cargo_amount);
    Ok(())
}

fn calc_and_set_mine_asteroid_amount(bot: &mut MiningBot) {
    bot.mine_asteroid_amount = calc_asteroid_mining_amount(
        bot.cargo_hold.1,
        bot.cargo_hold.2,
        bot.mine_asteroid_emission_rate,
        bot.mine_start(),
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

        // FIXME: need better dermination of when to start mining, i.g. food, fuel, ammo, etc.
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
    index: usize,
    sage: &sage::SageContext,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        get_and_set_token_balances_for_fuel_ammo_and_cargo(bot, sage)?;
        calc_and_set_mine_asteroid_amount(bot);
        calc_and_set_mine_asteroid_duration(bot);

        if bot.mine_asteroid_duraiton != Duration::ZERO {
            log::info!(
                "[{}] Prepare to start mining asteroid",
                &bot.masked_fleet_id()
            );

            bot.is_tx = true;
            if let Err(_e) = sage_tx.send(TxEvent::StartMiningAsteroid(index)) {
                bot.is_tx = false;
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
    index: usize,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    if bot.is_fleet_mining() {
        log::info!(
            "[{}] Prepare to stop mining asteroid",
            &bot.masked_fleet_id()
        );

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(TxEvent::StopMiningAsteroid(index)) {
            bot.is_tx = false;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_starbase_dock_handler(
    bot: &mut MiningBot,
    index: usize,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        log::info!("[{}] Prepare to dock to starbase", &bot.masked_fleet_id());

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(TxEvent::DockToStarbase(index)) {
            bot.is_tx = false;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_undock_starbase_dock_handler(
    bot: &mut MiningBot,
    index: usize,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    if bot.is_fleet_at_starbase() {
        log::info!(
            "[{}] Prepare to undock from starbase",
            &bot.masked_fleet_id()
        );

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(TxEvent::UndockFromStarbase(index)) {
            bot.is_tx = false;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_starbase_hangar_cargo_withdraw(
    bot: &mut MiningBot,
    index: usize,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    if let Some(_starbase) = bot.starbase_id() {
        log::info!("[{}] Prepare to widthraw mine item", &bot.masked_fleet_id());

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(TxEvent::StarbaseHangarCargoWithdraw(index)) {
            bot.is_tx = false;
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

fn autoplay_starbase_hangar_cargo_deposit(
    bot: &mut MiningBot,
    index: usize,
    deposit: CargoDeposit,
    sage: &sage::SageContext,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    if let Some(_starbase) = bot.starbase_id() {
        match deposit {
            CargoDeposit::Fuel => {
                // Fuel tank refuel
                get_and_set_token_balance_for_fuel(bot, sage)?;
                log::info!(
                    "[{}] Starbase Hangar check fuel tank {:?}",
                    &bot.masked_fleet_id(),
                    &bot.fuel_tank
                );

                let (_fuel_tank, actual, capacity) = &bot.fuel_tank.clone();

                if (*actual as f32 / *capacity as f32) < 0.5 {
                    log::info!("[{}] Prepare to refill fuel", &bot.masked_fleet_id());

                    bot.is_tx = true;
                    if let Err(_e) = sage_tx.send(TxEvent::StarbaseHangarDepositToFleet(
                        index,
                        CargoDeposit::Fuel,
                    )) {
                        bot.is_tx = false;
                    };
                } else {
                    bot.autoplay = Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Ammo);
                }
            }
            CargoDeposit::Ammo => {
                // Ammo bank rearm
                get_and_set_token_balance_for_ammo_bank(bot, sage)?;
                log::info!(
                    "[{}] Starbase Hangar check ammo bank {:?}",
                    &bot.masked_fleet_id(),
                    &bot.ammo_bank
                );

                let (_ammo_bank, actual, capacity) = &bot.ammo_bank.clone();

                if (*actual as f32 / *capacity as f32) < 0.5 {
                    log::info!("[{}] Prepare to rearm ammo", &bot.masked_fleet_id());

                    bot.is_tx = true;
                    if let Err(_e) = sage_tx.send(TxEvent::StarbaseHangarDepositToFleet(
                        index,
                        CargoDeposit::Ammo,
                    )) {
                        bot.is_tx = false;
                    };
                } else {
                    bot.autoplay = Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Food);
                }
            }
            CargoDeposit::Food => {
                // Cargo hold supply (food)
                get_and_set_token_balance_for_cargo_hold(bot, sage)?;
                log::info!(
                    "[{}] Starbase Hangar check food supply {:?}",
                    &bot.masked_fleet_id(),
                    &bot.cargo_hold
                );

                let (_cargo_hold, actual, capacity) = &bot.cargo_hold.clone();

                if (*actual as f32 / *capacity as f32) < 0.05 {
                    log::info!("[{}] Prepare to supply food", &bot.masked_fleet_id());

                    bot.is_tx = true;
                    if let Err(_e) = sage_tx.send(TxEvent::StarbaseHangarDepositToFleet(
                        index,
                        CargoDeposit::Food,
                    )) {
                        bot.is_tx = false;
                    };
                } else {
                    bot.autoplay = Autoplay::StarbaseUndock;
                }
            }
        };
    } else {
        bot.is_fleet_state_dirty = true;
    }

    Ok(())
}

pub fn run_autoplay(
    bot: &mut MiningBot,
    index: usize,
    dt: Duration,
    sage: &sage::SageContext,
    sage_tx: &Sender<TxEvent>,
) -> anyhow::Result<()> {
    bot.tick(dt);

    if bot.is_tx {
        return Ok(());
    }

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
            Autoplay::StarbaseHangarCargoDeposit(deposit) => match deposit {
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
        Autoplay::StartMiningAsteroid => {
            autoplay_start_mining_asteroid_handler(bot, index, sage, sage_tx)?
        }
        Autoplay::IsMiningAsteroid => autoplay_is_mining_asteroid_handler(bot)?,
        Autoplay::StopMiningAsteroid => autoplay_stop_mining_asteroid_handler(bot, index, sage_tx)?,
        Autoplay::StarbaseDock => autoplay_starbase_dock_handler(bot, index, sage_tx)?,
        Autoplay::StarbaseUndock => autoplay_undock_starbase_dock_handler(bot, index, sage_tx)?,
        Autoplay::StarbaseHangarCargoWithdraw => {
            autoplay_starbase_hangar_cargo_withdraw(bot, index, sage_tx)?
        }
        Autoplay::StarbaseHangarCargoDeposit(deposit) => {
            autoplay_starbase_hangar_cargo_deposit(bot, index, *deposit, sage, sage_tx)?
        }
    }

    Ok(())
}
