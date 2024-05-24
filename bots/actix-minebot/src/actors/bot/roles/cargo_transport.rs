use super::*;

pub(crate) fn clock_time_update(bot: &mut BotActor, msg: ClockTimeUpdate) {
    let clock = msg.0;

    match bot.role {
        BotRole::CargoTransport { .. } => match &bot.fleet_state() {
            FleetState::MoveWarp(move_warp) => {
                let time_elapsed = clock.unix_timestamp - move_warp.warp_start;
                let elapsed = std::time::Duration::from_secs_f64(time_elapsed as f64);

                let warp_duration = move_warp.warp_finish - move_warp.warp_start;
                let mut timer = timers::Timer::from_seconds(warp_duration as f32);
                timer.set_elapsed(elapsed);

                let (_, FleetWithState(fleet, _)) = &bot.fleet;
                let cooldown_duration = fleet.stats.movement_stats.warp_cool_down;
                let mut cooldown = timers::Timer::from_seconds(cooldown_duration as f32);
                cooldown.set_elapsed(elapsed);

                let operation = BotOps::Warp(WarpOps {
                    sector: move_warp.to_sector.clone(),
                    timer,
                    cooldown,
                });

                bot.operation = Some(operation);
            }
            FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                match &bot.operation {
                    Some(BotOps::StarbaseLoadingBay(_)) => {
                        // Do nothing, already performing a starbase loading operation
                    }
                    _ => {
                        let operation = BotOps::StarbaseLoadingBay(StarbaseLoadingBayOps {
                            starbase: starbase_loading_bay.starbase,
                            stopwatch: timers::Stopwatch::new(),
                            next_action: StarbaseActions::IdleHangar,
                        });

                        bot.operation = Some(operation);
                    }
                }
            }
            _ => {}
        },
        _ => unimplemented!(),
    }

    bot.clock = Some(clock);
}

pub(crate) fn sage_response(bot: &mut BotActor, msg: SageResponse, addr: Addr<BotActor>) {
    match &bot.role {
        BotRole::CargoTransport {
            cargo_mint,
            from_sector,
            to_sector,
            ..
        } => {
            let (_, FleetWithState(fleet, state)) = &mut bot.fleet;

            match msg {
                SageResponse::Fleet(FleetWithState(fleet, new_state)) => {
                    bot.operation = None;

                    match &new_state {
                        FleetState::Idle(_idle) => {
                            // fleet is "idle" request a check on cargo hold to determine next operation
                            bot.addr_sage
                                .do_send(SageRequest::FleetCargoHold(fleet.cargo_hold, addr));
                        }
                        FleetState::MoveWarp(_move_warp) => {
                            match &bot.operation {
                                Some(BotOps::Warp(_)) => {} // Do nothing, already performing a warping operation
                                _ => {
                                    // Request a "Clock" to kick-off the warping operation
                                    bot.addr_sage.do_send(ClockTimeRequest(addr));
                                }
                            }
                        }
                        FleetState::StarbaseLoadingBay(_starbase_loading_bay) => {
                            match &bot.operation {
                                Some(BotOps::StarbaseLoadingBay(_)) => {} // Do nothing, already performing a starbase loading operation
                                _ => {
                                    // Request a "FleetCargoHold" to kick-off the starbase loading operation
                                    bot.addr_sage.do_send(SageRequest::FleetCargoHold(
                                        fleet.cargo_hold,
                                        addr,
                                    ));
                                }
                            }
                        }
                        _ => {}
                    }

                    bot.fleet.1 = FleetWithState(fleet, new_state);
                }
                SageResponse::FleetCargoHold(cargo_hold) => match &state {
                    FleetState::Idle(idle) => {
                        let default = (cargo_mint.to_string(), 0);
                        let cargo = cargo_hold
                            .iter()
                            .find(|(mint, _)| mint == &cargo_mint.to_string())
                            .unwrap_or(&default);

                        let cargo_capacity_fraction =
                            cargo.1 as f64 / fleet.stats.cargo_stats.cargo_capacity as f64;

                        if from_sector == &idle.sector {
                            if cargo_capacity_fraction < 0.5 {
                                let ops = BotOps::Idle(IdleOps {
                                    sector: idle.sector.clone(),
                                    cargo_capacity_fraction,
                                    stopwatch: timers::Stopwatch::new(),
                                    next_action: IdleActions::DockeToStarbase,
                                });
                                bot.operation = Some(ops);
                            } else {
                                let ops = BotOps::Idle(IdleOps {
                                    sector: idle.sector.clone(),
                                    cargo_capacity_fraction,
                                    stopwatch: timers::Stopwatch::new(),
                                    next_action: IdleActions::WarpToSector(to_sector.clone()),
                                });
                                bot.operation = Some(ops);
                            }
                        }

                        if to_sector == &idle.sector {
                            if cargo_capacity_fraction > 0.5 {
                                let ops = BotOps::Idle(IdleOps {
                                    sector: idle.sector.clone(),
                                    cargo_capacity_fraction,
                                    stopwatch: timers::Stopwatch::new(),
                                    next_action: IdleActions::DockeToStarbase,
                                });
                                bot.operation = Some(ops);
                            } else {
                                let ops = BotOps::Idle(IdleOps {
                                    sector: idle.sector.clone(),
                                    cargo_capacity_fraction,
                                    stopwatch: timers::Stopwatch::new(),
                                    next_action: IdleActions::WarpToSector(from_sector.clone()),
                                });
                                bot.operation = Some(ops);
                            }
                        }
                    }
                    FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                        let default = (cargo_mint.to_string(), 0);
                        let cargo = cargo_hold
                            .iter()
                            .find(|(mint, _)| mint == &cargo_mint.to_string())
                            .unwrap_or(&default);

                        let cargo_capacity_fraction =
                            cargo.1 as f64 / fleet.stats.cargo_stats.cargo_capacity as f64;

                        let (cargo_mint, cargo_amount, from_starbase, to_starbase) = match &bot.role
                        {
                            BotRole::CargoTransport {
                                cargo_mint,
                                cargo_amount,
                                from_starbase,
                                to_starbase,
                                ..
                            } => (cargo_mint, cargo_amount, from_starbase, to_starbase),
                            _ => unimplemented!(),
                        };

                        let mut next_action = StarbaseActions::IdleHangar;

                        if &starbase_loading_bay.starbase == from_starbase {
                            if cargo_capacity_fraction < 0.5 {
                                next_action = StarbaseActions::CargoDeposit(
                                    fleet.cargo_hold,
                                    *cargo_mint,
                                    *cargo_amount,
                                );
                            } else {
                                next_action = StarbaseActions::UndockFromStarbase;
                            }
                        }

                        if &starbase_loading_bay.starbase == to_starbase {
                            if cargo_capacity_fraction > 0.5 {
                                next_action =
                                    StarbaseActions::CargoWithdraw(*cargo_mint, *cargo_amount);
                            } else {
                                next_action = StarbaseActions::CheckFuelStatus;
                            }
                        }

                        let starbase_ops = match &bot.operation {
                            Some(BotOps::StarbaseLoadingBay(starbase_ops)) => {
                                let mut starbase_ops = starbase_ops.clone();
                                starbase_ops.next_action = next_action;
                                starbase_ops
                            }
                            _ => StarbaseLoadingBayOps {
                                starbase: starbase_loading_bay.starbase,
                                stopwatch: timers::Stopwatch::new(),
                                next_action,
                            },
                        };

                        bot.operation = Some(BotOps::StarbaseLoadingBay(starbase_ops));
                    }
                    _ => {}
                },
                SageResponse::FleetFuelTank(fuel_tank) => {
                    bot.fleet_fuel_tank = fuel_tank;

                    match state {
                        FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                            let (fuel_mint, fuel_amount) = &bot.fleet_fuel_tank[0];
                            let fuel_tank_fraction =
                                *fuel_amount as f32 / fleet.stats.cargo_stats.fuel_capacity as f32;

                            let next_action = if fuel_tank_fraction < 0.5 {
                                let amount =
                                    fleet.stats.cargo_stats.fuel_capacity as u64 - *fuel_amount;

                                let fuel_mint = Pubkey::from_str(&fuel_mint).unwrap();
                                StarbaseActions::CargoDeposit(fleet.fuel_tank, fuel_mint, amount)
                            } else {
                                StarbaseActions::UndockFromStarbase
                            };

                            let starbase_ops = match &bot.operation {
                                Some(BotOps::StarbaseLoadingBay(starbase_ops)) => {
                                    let mut starbase_ops = starbase_ops.clone();
                                    starbase_ops.next_action = next_action;
                                    starbase_ops
                                }
                                _ => StarbaseLoadingBayOps {
                                    starbase: starbase_loading_bay.starbase,
                                    stopwatch: timers::Stopwatch::new(),
                                    next_action,
                                },
                            };

                            bot.operation = Some(BotOps::StarbaseLoadingBay(starbase_ops));
                        }
                        _ => {}
                    }
                }
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn tick(bot: &mut BotActor, msg: Tick, addr: Addr<BotActor>) {
    let (fleet_id, FleetWithState(fleet, state)) = &bot.fleet;
    let fleet = (*fleet_id, *fleet);

    match &mut bot.operation {
        Some(BotOps::Idle(ops)) => {
            ops.stopwatch.tick(msg.0);
            log::info!("{:#?}", &ops);

            match ops.next_action {
                IdleActions::DockeToStarbase => {
                    bot.addr_sage
                        .do_send(SageAction::StarbaseDock(fleet, ops.sector, addr));

                    bot.operation =
                        Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        }));
                }
                IdleActions::WarpToSector(sector) => {
                    bot.addr_sage.do_send(SageAction::Warp(fleet, sector, addr));

                    bot.operation =
                        Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        }));
                }
                _ => unimplemented!("{:?}", ops.next_action),
            }
        }
        Some(BotOps::TxsSageBased(ops)) => {
            ops.stopwatch.tick(msg.0);
            log::info!("{:#?}", &ops);
        }
        Some(BotOps::Warp(ops)) => {
            ops.timer.tick(msg.0);
            ops.cooldown.tick(msg.0);

            match state {
                // FleetState::Idle(_) => {
                //     bot.addr_sage
                //         .do_send(SageAction::Warp(fleet, ops.sector.clone(), addr));

                //     bot.operation =
                //         Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                //             stopwatch: timers::Stopwatch::new(),
                //         }));
                // }
                FleetState::MoveWarp(_) => {
                    if ops.timer.finished() && ops.cooldown.finished() {
                        bot.addr_sage.do_send(SageAction::WarpExit(fleet, addr));

                        bot.operation =
                            Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                stopwatch: timers::Stopwatch::new(),
                            }));
                    }
                }
                _ => unimplemented!("{:?}", state),
            }
        }
        Some(BotOps::StarbaseLoadingBay(ops)) => {
            ops.stopwatch.tick(msg.0);
            log::info!("{:#?}", &ops);

            match ops.next_action {
                StarbaseActions::IdleHangar => {
                    // Request a "FleetCargoHold" to kick-off the starbase loading operation
                    bot.addr_sage
                        .do_send(SageRequest::FleetCargoHold(fleet.1.cargo_hold, addr));
                }
                StarbaseActions::CargoDeposit(cargo_pod_to, mint, amount) => {
                    bot.addr_sage.do_send(SageAction::CargoDeposit(
                        fleet,
                        ops.starbase,
                        cargo_pod_to,
                        mint,
                        amount,
                        addr,
                    ));

                    bot.operation =
                        Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        }));
                }
                StarbaseActions::CargoWithdraw(mint, amount) => {
                    bot.addr_sage.do_send(SageAction::CargoWithdraw(
                        fleet,
                        ops.starbase,
                        mint,
                        amount,
                        addr,
                    ));

                    bot.operation =
                        Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        }));
                }
                StarbaseActions::CheckFuelStatus => {
                    bot.addr_sage
                        .do_send(SageRequest::FleetFuelTank(fleet.1.fuel_tank, addr));
                }
                StarbaseActions::UndockFromStarbase => {
                    bot.addr_sage
                        .do_send(SageAction::StarbaseUndock(fleet, ops.starbase, addr));

                    bot.operation =
                        Some(autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        }));
                }
                _ => unimplemented!("{:?}", ops.next_action),
            }
        }
        None => {
            // if operation is None, request the fleet state to kick-off the bot
            bot.addr_sage.do_send(SageRequest::Fleet(*fleet_id, addr));
        }
        _ => unimplemented!("{:?}", bot.operation),
    }
}
