use std::sync::mpsc::Sender;

use super::*;

fn autoplay_is_idle_handler(bot: &mut MiningBot) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        let actual = bot.cargo_hold_amount as f32;
        let capacity = bot.cargo_hold_capacity as f32;

        // FIXME: need better dermination of when to start mining, i.g. food, fuel, ammo, etc.
        if (actual / capacity) <= 0.9 {
            bot.autoplay = Autoplay::StartMiningAsteroid;
        } else {
            bot.autoplay = Autoplay::StarbaseDock;
        }
    }

    Ok(())
}

fn autoplay_start_mining_asteroid_handler(
    bot: &mut MiningBot,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        if bot.mine_asteroid_duraiton != Duration::ZERO {
            log::info!(
                "[{}] Prepare to start mining asteroid",
                &bot.masked_fleet_id()
            );

            bot.is_tx = true;
            if let Err(_e) = sage_tx.send(labs::SageRequest::StartMiningAsteroid(bot.fleet())) {
                bot.is_tx = false;
            };
        } else {
            bot.autoplay = Autoplay::IsIdle;
        }
    }

    Ok(())
}

fn autoplay_is_mining_asteroid_handler(bot: &mut MiningBot) -> anyhow::Result<()> {
    if bot.is_fleet_mining() {
        if bot.mining_timer.finished() {
            bot.autoplay = Autoplay::StopMiningAsteroid;
        }
    }

    Ok(())
}

fn autoplay_stop_mining_asteroid_handler(
    bot: &mut MiningBot,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    if bot.is_fleet_mining() {
        log::info!(
            "[{}] Prepare to stop mining asteroid",
            &bot.masked_fleet_id()
        );

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(labs::SageRequest::StopMiningAsteroid(bot.fleet())) {
            bot.is_tx = false;
        };
    }

    Ok(())
}

fn autoplay_starbase_dock_handler(
    bot: &mut MiningBot,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    if bot.is_fleet_idle() {
        log::info!("[{}] Prepare to dock to starbase", &bot.masked_fleet_id());

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(labs::SageRequest::DockToStarbase(bot.fleet())) {
            bot.is_tx = false;
        };
    }

    Ok(())
}

fn autoplay_undock_starbase_dock_handler(
    bot: &mut MiningBot,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    if bot.is_fleet_at_starbase() {
        log::info!(
            "[{}] Prepare to undock from starbase",
            &bot.masked_fleet_id()
        );

        bot.is_tx = true;
        if let Err(_e) = sage_tx.send(labs::SageRequest::UndockFromStarbase(bot.fleet())) {
            bot.is_tx = false;
        };
    }

    Ok(())
}

fn autoplay_starbase_hangar_cargo_withdraw(
    bot: &mut MiningBot,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    if let Some(_starbase) = bot.starbase_id() {
        log::info!("[{}] Prepare to widthraw mine item", &bot.masked_fleet_id());

        bot.is_tx = true;

        let (fleet_id, fleet, _) = bot.fleet();
        let starbase_id = bot.starbase_id().unwrap();
        let mine_item = &bot.mine_item.1;
        let mint = &mine_item.mint;

        if let Err(_e) = sage_tx.send(labs::SageRequest::StarbaseHangarCargoWithdraw((
            fleet_id,
            fleet,
            *starbase_id,
            *mint,
        ))) {
            bot.is_tx = false;
        };
    }

    Ok(())
}

fn autoplay_starbase_hangar_cargo_deposit(
    bot: &mut MiningBot,
    deposit: CargoDeposit,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    if let Some(_starbase) = bot.starbase_id() {
        match deposit {
            CargoDeposit::Fuel => {
                // Fuel tank refuel
                log::info!(
                    "[{}] Starbase Hangar check fuel tank {:?}",
                    &bot.masked_fleet_id(),
                    &bot.fuel_tank
                );

                let fuel_tank = &bot.fuel_tank;
                let actual = bot.fuel_tank_amount as f32;
                let capacity = bot.fuel_tank_capacity as f32;

                if (actual / capacity) < 0.5 {
                    log::info!("[{}] Prepare to refill fuel", &bot.masked_fleet_id());

                    bot.is_tx = true;

                    let (fleet_id, fleet, _) = bot.fleet();
                    let starbase_id = bot.starbase_id().unwrap();
                    let cargo_deposit = CargoDeposit::Fuel;
                    let amount = (capacity - actual) as u64;

                    if let Err(_e) = sage_tx.send(labs::SageRequest::StarbaseHangarDepositToFleet(
                        fleet_id,
                        fleet,
                        *starbase_id,
                        *fuel_tank,
                        cargo_deposit,
                        amount,
                    )) {
                        bot.is_tx = false;
                    };
                } else {
                    bot.autoplay = Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Ammo);
                }
            }
            CargoDeposit::Ammo => {
                // Ammo bank rearm
                log::info!(
                    "[{}] Starbase Hangar check ammo bank {:?}",
                    &bot.masked_fleet_id(),
                    &bot.ammo_bank
                );

                let ammo_bank = &bot.ammo_bank;
                let actual = bot.ammo_bank_amount as f32;
                let capacity = bot.ammo_bank_capacity as f32;

                if (actual / capacity) < 0.5 {
                    log::info!("[{}] Prepare to rearm ammo", &bot.masked_fleet_id());

                    bot.is_tx = true;

                    let (fleet_id, fleet, _) = bot.fleet();
                    let starbase_id = bot.starbase_id().unwrap();
                    let cargo_deposit = CargoDeposit::Ammo;
                    let amount = (capacity - actual) as u64;

                    if let Err(_e) = sage_tx.send(labs::SageRequest::StarbaseHangarDepositToFleet(
                        fleet_id,
                        fleet,
                        *starbase_id,
                        *ammo_bank,
                        cargo_deposit,
                        amount,
                    )) {
                        bot.is_tx = false;
                    };
                } else {
                    bot.autoplay = Autoplay::StarbaseHangarCargoDeposit(CargoDeposit::Food);
                }
            }
            CargoDeposit::Food => {
                // Cargo hold supply (food)
                log::info!(
                    "[{}] Starbase Hangar check food supply {:?}",
                    &bot.masked_fleet_id(),
                    &bot.cargo_hold
                );

                let cargo_hold = &bot.cargo_hold;
                let actual = bot.cargo_hold_amount as f32;
                let capacity = bot.cargo_hold_capacity as f32;

                if (actual / capacity) < 0.05 {
                    log::info!("[{}] Prepare to supply food", &bot.masked_fleet_id());

                    bot.is_tx = true;

                    let (fleet_id, fleet, _) = bot.fleet();
                    let starbase_id = bot.starbase_id().unwrap();
                    let cargo_deposit = CargoDeposit::Food;
                    let amount = (capacity * 0.05) as u64;

                    if let Err(_e) = sage_tx.send(labs::SageRequest::StarbaseHangarDepositToFleet(
                        fleet_id,
                        fleet,
                        *starbase_id,
                        *cargo_hold,
                        cargo_deposit,
                        amount,
                    )) {
                        bot.is_tx = false;
                    };
                } else {
                    bot.autoplay = Autoplay::StarbaseUndock;
                }
            }
        };
    }

    Ok(())
}

pub fn autoplay(
    bot: &mut MiningBot,
    dt: Duration,
    sage_tx: &Sender<labs::SageRequest>,
) -> anyhow::Result<()> {
    bot.tick(dt);

    if bot.is_tx {
        return Ok(());
    }

    match &bot.autoplay {
        Autoplay::IsIdle => autoplay_is_idle_handler(bot)?,
        Autoplay::StartMiningAsteroid => autoplay_start_mining_asteroid_handler(bot, sage_tx)?,
        Autoplay::IsMiningAsteroid => autoplay_is_mining_asteroid_handler(bot)?,
        Autoplay::StopMiningAsteroid => autoplay_stop_mining_asteroid_handler(bot, sage_tx)?,
        Autoplay::StarbaseDock => autoplay_starbase_dock_handler(bot, sage_tx)?,
        Autoplay::StarbaseUndock => autoplay_undock_starbase_dock_handler(bot, sage_tx)?,
        Autoplay::StarbaseHangarCargoWithdraw => {
            autoplay_starbase_hangar_cargo_withdraw(bot, sage_tx)?
        }
        Autoplay::StarbaseHangarCargoDeposit(deposit) => {
            autoplay_starbase_hangar_cargo_deposit(bot, *deposit, sage_tx)?
        }
    }

    Ok(())
}
