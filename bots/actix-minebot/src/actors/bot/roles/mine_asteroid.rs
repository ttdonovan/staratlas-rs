use super::*;

pub(crate) fn clock_time_update(bot: &mut BotActor, msg: ClockTimeUpdate) {
    match &bot.role {
        BotRole::MineAsteroid {
            planet,
            mine_item,
            resource,
        } => {
            let clock = msg.0;

            match &bot.fleet_state() {
                FleetState::MineAsteroid(mine_asteroid) => {
                    let mining_ops = autoplay_mine_asteroid(
                        &bot,
                        &mine_asteroid,
                        &clock,
                        &planet,
                        &mine_item,
                        &resource,
                    );
                    let operation = autoplay::BotOps::Mining(mining_ops);
                    bot.operation = Some(operation);
                }
                _ => {}
            }

            bot.clock = Some(clock);
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn sage_response(bot: &mut BotActor, msg: SageResponse, addr: Addr<BotActor>) {
    match &bot.role {
        BotRole::MineAsteroid { mine_item, .. } => match msg {
            SageResponse::Fleet(fleet_with_state) => {
                autoplay_fleet_with_state_update(bot, fleet_with_state, addr);
            }
            SageResponse::FleetAmmoBank(ammo_bank) => {
                bot.fleet_ammo_bank = ammo_bank;

                match bot.fleet_state() {
                    FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                        let starbase_loading_bay_ops =
                            autoplay_starbase_loading(&bot, starbase_loading_bay, mine_item);

                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        bot.operation = Some(operation);
                    }
                    _ => {}
                }
            }
            SageResponse::FleetCargoHold(cargo_hold) => {
                bot.fleet_cargo_hold = cargo_hold;

                match bot.fleet_state() {
                    FleetState::Idle(idle) => {
                        let idle_ops = autoplay_idle(&bot, idle);
                        let operation = autoplay::BotOps::Idle(idle_ops);

                        bot.operation = Some(operation);
                    }
                    FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                        let starbase_loading_bay_ops =
                            autoplay_starbase_loading(&bot, starbase_loading_bay, mine_item);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        bot.operation = Some(operation);
                    }
                    _ => {}
                }
            }
            SageResponse::FleetFuelTank(fuel_tank) => {
                bot.fleet_fuel_tank = fuel_tank;

                match bot.fleet_state() {
                    FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                        let starbase_loading_bay_ops =
                            autoplay_starbase_loading(&bot, starbase_loading_bay, mine_item);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        bot.operation = Some(operation);
                    }
                    _ => {}
                }
            }
            SageResponse::FleetFoodCargoHold(food_cargo) => {
                bot.fleet_food_cargo = food_cargo;

                match &bot.fleet_state() {
                    FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                        let starbase_loading_bay_ops =
                            autoplay_starbase_loading(&bot, starbase_loading_bay, mine_item);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        bot.operation = Some(operation);
                    }
                    _ => {}
                }
            }
        },
        _ => unimplemented!(),
    }
}

pub(crate) fn tick(bot: &mut BotActor, msg: Tick, addr: Addr<BotActor>) {
    match &bot.role {
        BotRole::MineAsteroid {
            planet,
            mine_item,
            resource,
        } => {
            let (fleet_id, FleetWithState(fleet, _)) = bot.fleet;
            let fleet = (fleet_id, fleet);

            if let Some(operation) = &mut bot.operation {
                match operation {
                    autoplay::BotOps::TxsSageBased(txs_sage_based_ops) => {
                        txs_sage_based_ops.stopwatch.tick(msg.0);
                        log::info!("{:#?}", &txs_sage_based_ops);
                    }
                    autoplay::BotOps::Idle(idle_ops) => {
                        idle_ops.stopwatch.tick(msg.0);
                        log::info!("{:#?}", &idle_ops);

                        let sector = idle_ops.sector;

                        match idle_ops.next_action {
                            autoplay::IdleActions::DockeToStarbase => {
                                bot.addr_sage
                                    .do_send(SageAction::StarbaseDock(fleet, sector, addr));

                                let operation =
                                    autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                        stopwatch: timers::Stopwatch::new(),
                                    });
                                bot.operation = Some(operation);
                            }
                            autoplay::IdleActions::MineAsteroid => {
                                let planet = planet.0;
                                let mine_item = mine_item.0;
                                let resource = resource.0;

                                bot.addr_sage.do_send(SageAction::StartMining(
                                    fleet, mine_item, resource, planet, sector, addr,
                                ));

                                let operation =
                                    autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                        stopwatch: timers::Stopwatch::new(),
                                    });
                                bot.operation = Some(operation);
                            }
                            _ => unimplemented!("{:?}", idle_ops.next_action),
                        }
                    }
                    autoplay::BotOps::Mining(mining_ops) => {
                        mining_ops.timer.tick(msg.0);
                        log::info!("{:#?}", &mining_ops);

                        if mining_ops.timer.finished() {
                            let planet_id = planet.0;
                            let mine_item_id = mine_item.0;
                            let mine_item_mint = mine_item.1.mint;
                            let resource = resource.0;
                            let sector = planet.1.sector;

                            bot.addr_sage.do_send(SageAction::StopMining(
                                fleet,
                                mine_item_id,
                                mine_item_mint,
                                resource,
                                planet_id,
                                sector,
                                addr,
                            ));

                            let operation =
                                autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                    stopwatch: timers::Stopwatch::new(),
                                });
                            bot.operation = Some(operation);
                        }
                    }
                    autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops) => {
                        starbase_loading_bay_ops.stopwatch.tick(msg.0);
                        log::info!("{:#?}", &starbase_loading_bay_ops);

                        match starbase_loading_bay_ops.next_action {
                            autoplay::StarbaseActions::IdleHangar => {
                                unimplemented!("{:?}", starbase_loading_bay_ops.next_action)
                            }
                            autoplay::StarbaseActions::CargoDeposit(cargo_pod_to, mint, amount) => {
                                bot.addr_sage.do_send(SageAction::CargoDeposit(
                                    fleet,
                                    starbase_loading_bay_ops.starbase,
                                    cargo_pod_to,
                                    mint,
                                    amount,
                                    addr,
                                ));

                                let operation =
                                    autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                        stopwatch: timers::Stopwatch::new(),
                                    });
                                bot.operation = Some(operation);
                            }
                            autoplay::StarbaseActions::CargoWithdraw(mint, amount) => {
                                bot.addr_sage.do_send(SageAction::CargoWithdraw(
                                    fleet,
                                    starbase_loading_bay_ops.starbase,
                                    mint,
                                    amount,
                                    addr,
                                ));

                                let operation =
                                    autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                        stopwatch: timers::Stopwatch::new(),
                                    });
                                bot.operation = Some(operation);
                            }
                            autoplay::StarbaseActions::CheckFuelStatus => {
                                bot.addr_sage
                                    .do_send(SageRequest::FleetFuelTank(fleet.1.fuel_tank, addr));
                            }
                            autoplay::StarbaseActions::CheckAmmoStatus => {
                                bot.addr_sage
                                    .do_send(SageRequest::FleetAmmoBank(fleet.1.ammo_bank, addr));
                            }
                            autoplay::StarbaseActions::CheckFoodStatus => {
                                bot.addr_sage.do_send(SageRequest::FleetFoodCargoHold(
                                    fleet.1.cargo_hold,
                                    addr,
                                ));
                            }
                            autoplay::StarbaseActions::UndockFromStarbase => {
                                bot.addr_sage.do_send(SageAction::StarbaseUndock(
                                    fleet,
                                    starbase_loading_bay_ops.starbase,
                                    addr,
                                ));

                                let operation =
                                    autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                        stopwatch: timers::Stopwatch::new(),
                                    });
                                bot.operation = Some(operation);
                            }
                        }
                    }
                    _ => unimplemented!("{:?}", operation),
                }
            } else {
                bot.addr_sage.do_send(SageRequest::Fleet(fleet_id, addr));
            }
        }
        _ => unimplemented!(),
    }
}

fn autoplay_fleet_with_state_update(
    bot: &mut BotActor,
    fleet_with_state: FleetWithState,
    addr: Addr<BotActor>,
) {
    bot.operation = None;
    let FleetWithState(fleet, fleet_state) = &fleet_with_state;

    match fleet_state {
        FleetState::Idle(_idle) => {
            bot.addr_sage
                .do_send(SageRequest::FleetCargoHold(fleet.cargo_hold, addr));
        }
        FleetState::MineAsteroid(_mine_asteroid) => {
            match &bot.operation {
                Some(BotOps::Mining(_)) => {} // Do nothing, already performing a mining operation
                _ => {
                    // Request a "Clock" to kick-off the mining operation
                    bot.addr_sage.do_send(ClockTimeRequest(addr));
                }
            }
        }
        FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
            log::info!("{:?}", starbase_loading_bay);

            match &bot.operation {
                Some(BotOps::StarbaseLoadingBay(_)) => {
                    // Do nothing, already performing a starbase loading bay operation
                    // 1. Unload Cargo
                    // 2. Resupply Fuel
                    // 3. Resupply Ammo
                    // 4. Resupply Food
                }
                _ => {
                    // Request an update on the fleet's cargo hold to kick-off the starbase loading bay operation
                    bot.addr_sage
                        .do_send(SageRequest::FleetCargoHold(fleet.cargo_hold, addr));
                }
            }
        }
        _ => {}
    }

    bot.fleet.1 = fleet_with_state;
}

fn autoplay_idle(bot: &BotActor, idle: &Idle) -> IdleOps {
    let (_, FleetWithState(fleet, _)) = &bot.fleet;
    let current_capacity = bot.fleet_cargo_hold.iter().fold(0, |x, (_, v)| x + v);
    let cargo_capacity_fraction =
        current_capacity as f64 / fleet.stats.cargo_stats.cargo_capacity as f64;

    let next_action = if cargo_capacity_fraction > 0.55 {
        IdleActions::DockeToStarbase
    } else {
        IdleActions::MineAsteroid
    };

    IdleOps {
        sector: idle.sector.clone(),
        cargo_capacity_fraction,
        stopwatch: timers::Stopwatch::new(),
        next_action,
    }
}

fn autoplay_mine_asteroid(
    bot: &BotActor,
    mine_asteroid: &MineAsteroid,
    clock: &Clock,
    planet: &(Pubkey, Planet),
    mine_item: &(Pubkey, MineItem),
    resource: &(Pubkey, Resource),
) -> MiningOps {
    let (_, FleetWithState(fleet, _)) = &bot.fleet;
    // let planet = &bot.planet.1;
    // let mine_item = &bot.mine_item.1;
    // let resource = &bot.resource.1;

    let mining_location = planet.1.name();
    let currently_mining = mine_item.1.name();

    let mining_rate = calc::asteroid_mining_emission_rate(&fleet.stats, &mine_item.1, &resource.1);

    let cargo_space = fleet.stats.cargo_stats.cargo_capacity;

    let mining_duration = calc::asteroid_mining_resource_extraction_duration(
        &fleet.stats,
        &mine_item.1,
        &resource.1,
        cargo_space,
    );

    let time_elapsed = clock.unix_timestamp - mine_asteroid.start;
    let amount_mined = time_elapsed as f32 * mining_rate;

    let mut mining_timer = timers::Timer::from_seconds(mining_duration);
    let elapsed = std::time::Duration::from_secs_f64(time_elapsed as f64);
    mining_timer.set_elapsed(elapsed);

    MiningOps {
        mining_location: mining_location.into(),
        currently_mining: currently_mining.into(),
        resource_mining_rate_per_second: mining_rate,
        amount_mined,
        timer: mining_timer,
    }
}

fn autoplay_starbase_loading(
    bot: &BotActor,
    starbase_loading_bay: &StarbaseLoadingBay,
    mine_item: &(Pubkey, MineItem),
) -> StarbaseLoadingBayOps {
    let mine_item_mint = mine_item.1.mint;

    let cargo_withdraw_amount = bot
        .fleet_cargo_hold
        .iter()
        .find_map(|(mint, amount)| {
            if mint == &mine_item_mint.to_string() {
                let amount = *amount;
                if amount > 1 {
                    Some(amount - 1)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(0 as u64);

    let next_action = if cargo_withdraw_amount > 0 {
        StarbaseActions::CargoWithdraw(mine_item_mint, cargo_withdraw_amount)
    } else {
        match &bot.operation {
            Some(BotOps::StarbaseLoadingBay(starbase_loading_bay_ops)) => {
                match starbase_loading_bay_ops.next_action {
                    StarbaseActions::CheckFuelStatus => {
                        let (_, FleetWithState(fleet, _)) = &bot.fleet;
                        let (fuel_mint, fuel_amount) = &bot.fleet_fuel_tank[0];
                        let fuel_tank_fraction =
                            *fuel_amount as f32 / fleet.stats.cargo_stats.fuel_capacity as f32;

                        if fuel_tank_fraction < 0.5 {
                            let amount =
                                fleet.stats.cargo_stats.fuel_capacity as u64 - *fuel_amount;

                            let fuel_mint = Pubkey::from_str(&fuel_mint).unwrap();
                            StarbaseActions::CargoDeposit(fleet.fuel_tank, fuel_mint, amount)
                        } else {
                            if fleet.stats.cargo_stats.ammo_consumption_rate == 0 {
                                StarbaseActions::CheckFoodStatus
                            } else {
                                StarbaseActions::CheckAmmoStatus
                            }
                        }
                    }
                    StarbaseActions::CheckAmmoStatus => {
                        let (_, FleetWithState(fleet, _)) = &bot.fleet;
                        let (ammo_mint, ammo_amount) = &bot.fleet_ammo_bank[0];
                        let ammo_bank_fraction =
                            *ammo_amount as f32 / fleet.stats.cargo_stats.ammo_capacity as f32;

                        if ammo_bank_fraction < 0.5 {
                            let amount =
                                fleet.stats.cargo_stats.ammo_capacity as u64 - *ammo_amount;

                            let ammo_mint = Pubkey::from_str(&ammo_mint).unwrap();
                            StarbaseActions::CargoDeposit(fleet.ammo_bank, ammo_mint, amount)
                        } else {
                            StarbaseActions::CheckFoodStatus
                        }
                    }
                    StarbaseActions::CheckFoodStatus => {
                        let (_, FleetWithState(fleet, _)) = &bot.fleet;
                        let (food_mint, food_amount) = &bot.fleet_food_cargo[0];
                        let min_food =
                            (fleet.stats.cargo_stats.cargo_capacity as f32 * 0.075) as u64;

                        if *food_amount < min_food {
                            let amount = min_food - *food_amount;

                            let food_mint = Pubkey::from_str(&food_mint).unwrap();
                            StarbaseActions::CargoDeposit(fleet.cargo_hold, food_mint, amount)
                        } else {
                            StarbaseActions::UndockFromStarbase
                        }
                    }
                    _ => StarbaseActions::CheckFuelStatus,
                }
            }
            _ => StarbaseActions::CheckFuelStatus,
        }
    };

    let stopwatch = match &bot.operation {
        Some(BotOps::StarbaseLoadingBay(starbase_loading_bay_ops)) => {
            starbase_loading_bay_ops.stopwatch.clone()
        }
        _ => timers::Stopwatch::new(),
    };

    StarbaseLoadingBayOps {
        starbase: starbase_loading_bay.starbase.clone(),
        stopwatch,
        next_action,
    }
}
