use super::*;

pub(crate) fn clock_time_update(bot: &mut BotActor, msg: ClockTimeUpdate) {
    let clock = msg.0;

    match bot.role {
        BotRole::CargoTransport { .. } => match &bot.fleet_state() {
            FleetState::MoveWarp(move_warp) => {
                let time_elapsed = clock.unix_timestamp - move_warp.warp_start;
                let elapsed = std::time::Duration::from_secs_f64(time_elapsed as f64);

                let warp_duration = move_warp.warp_finish - move_warp.warp_start;
                let mut warp_timer = timers::Timer::from_seconds(warp_duration as f32);
                warp_timer.set_elapsed(elapsed);

                let operation = BotOps::Warp(WarpOps {
                    sector: move_warp.to_sector.clone(),
                    timer: warp_timer,
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
                                let ops = BotOps::Warp(WarpOps {
                                    sector: to_sector.clone(),
                                    timer: timers::Timer::default(),
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
                                let ops = BotOps::Warp(WarpOps {
                                    sector: from_sector.clone(),
                                    timer: timers::Timer::default(),
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

    let warp_cool_down_timer = match &mut bot.role {
        BotRole::CargoTransport {
            warp_cool_down_timer,
            ..
        } => warp_cool_down_timer,
        _ => unimplemented!(),
    };

    if let Some(warp_cool_down_timer) = warp_cool_down_timer {
        warp_cool_down_timer.tick(msg.0);
    }

    if let Some(operation) = &mut bot.operation {
        match operation {
            BotOps::TxsSageBased(txs_sage_based_ops) => {
                txs_sage_based_ops.stopwatch.tick(msg.0);
                log::info!("{:#?}", &txs_sage_based_ops);
            }
            BotOps::Idle(idle_ops) => {
                idle_ops.stopwatch.tick(msg.0);
                log::info!("{:#?}", &idle_ops);

                match idle_ops.next_action {
                    IdleActions::DockeToStarbase => {
                        bot.addr_sage.do_send(SageAction::StarbaseDock(
                            fleet,
                            idle_ops.sector,
                            addr,
                        ));

                        let operation = BotOps::TxsSageBased(TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });
                        bot.operation = Some(operation);
                    }
                    _ => unimplemented!("{:?}", idle_ops.next_action),
                }
            }
            BotOps::Warp(warp_ops) => match state {
                FleetState::Idle(_) => {
                    if let Some(warp_cool_down_timer) = warp_cool_down_timer {
                        if warp_cool_down_timer.finished() {
                            bot.addr_sage.do_send(SageAction::Warp(
                                fleet,
                                warp_ops.sector.clone(),
                                addr,
                            ));

                            let operation = BotOps::TxsSageBased(TxsSageBasedOps {
                                stopwatch: timers::Stopwatch::new(),
                            });

                            bot.operation = Some(operation);
                        }
                    } else {
                        bot.addr_sage.do_send(SageAction::Warp(
                            fleet,
                            warp_ops.sector.clone(),
                            addr,
                        ));

                        let operation = BotOps::TxsSageBased(TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });

                        bot.operation = Some(operation);
                    }
                }
                FleetState::MoveWarp(_) => {
                    warp_ops.timer.tick(msg.0);

                    if warp_ops.timer.finished() {
                        let timer = timers::Timer::from_seconds(
                            fleet.1.stats.movement_stats.warp_cool_down as f32,
                        );
                        *warp_cool_down_timer = Some(timer);

                        bot.addr_sage.do_send(SageAction::WarpExit(fleet, addr));

                        let operation = BotOps::TxsSageBased(TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });
                        bot.operation = Some(operation);
                    }
                }
                _ => unimplemented!("{:?}", state),
            },
            BotOps::StarbaseLoadingBay(starbase_ops) => {
                starbase_ops.stopwatch.tick(msg.0);
                log::info!("{:#?}", &starbase_ops);

                match starbase_ops.next_action {
                    StarbaseActions::IdleHangar => {
                        // Request a "FleetCargoHold" to kick-off the starbase loading operation
                        bot.addr_sage
                            .do_send(SageRequest::FleetCargoHold(fleet.1.cargo_hold, addr));
                    }
                    StarbaseActions::CargoDeposit(cargo_pod_to, mint, amount) => {
                        bot.addr_sage.do_send(SageAction::CargoDeposit(
                            fleet,
                            starbase_ops.starbase,
                            cargo_pod_to,
                            mint,
                            amount,
                            addr,
                        ));

                        let operation = BotOps::TxsSageBased(TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });
                        bot.operation = Some(operation);
                    }
                    StarbaseActions::CargoWithdraw(mint, amount) => {
                        bot.addr_sage.do_send(SageAction::CargoWithdraw(
                            fleet,
                            starbase_ops.starbase,
                            mint,
                            amount,
                            addr,
                        ));

                        let operation = autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });
                        bot.operation = Some(operation);
                    }
                    StarbaseActions::CheckFuelStatus => {
                        bot.addr_sage
                            .do_send(SageRequest::FleetFuelTank(fleet.1.fuel_tank, addr));
                    }
                    StarbaseActions::UndockFromStarbase => {
                        bot.addr_sage.do_send(SageAction::StarbaseUndock(
                            fleet,
                            starbase_ops.starbase,
                            addr,
                        ));

                        let operation = autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });
                        bot.operation = Some(operation);
                    }
                    _ => unimplemented!("{:?}", starbase_ops.next_action),
                }
            }
            _ => unimplemented!("{:?}", operation),
        }
    } else {
        bot.addr_sage.do_send(SageRequest::Fleet(*fleet_id, addr));
    }
}
