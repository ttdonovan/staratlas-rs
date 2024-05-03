use super::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick(pub tokio::time::Duration);

impl Handler<Tick> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: Tick, ctx: &mut Context<Self>) {
        // log::info!("Tick {:?}", msg.0);

        {
            if let (Some(db), Some(data)) = (
                self.db.lock().ok(),
                serde_json::to_string(&self.operation).ok(),
            ) {
                db.conn
                    .execute(
                        "INSERT OR REPLACE INTO bot_ops (pubkey, data) VALUES (?1, ?2)",
                        rusqlite::params![self.fleet.0.to_string(), data],
                    )
                    .ok();
            }
        }

        match &mut self.operation {
            Some(operation) => match operation {
                autoplay::BotOps::TxsSageBased(txs_sage_based_ops) => {
                    txs_sage_based_ops.stopwatch.tick(msg.0);
                    log::info!("{:#?}", &txs_sage_based_ops);
                }
                autoplay::BotOps::Idle(idle_ops) => {
                    idle_ops.stopwatch.tick(msg.0);
                    log::info!("{:#?}", &idle_ops);

                    let fleet = self.fleet;
                    let sector = idle_ops.sector;

                    match idle_ops.next_action {
                        autoplay::IdleActions::DockeToStarbase => {
                            self.addr_sage.do_send(SageAction::StarbaseDock(
                                fleet,
                                sector,
                                ctx.address(),
                            ));

                            let operation =
                                autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                    stopwatch: timers::Stopwatch::new(),
                                });
                            self.operation = Some(operation);
                        }
                        autoplay::IdleActions::MineAsteroid => {
                            let planet = self.planet.0;
                            let mine_item = self.mine_item.0;
                            let resource = self.resource.0;

                            self.addr_sage.do_send(SageAction::StartMining(
                                fleet,
                                mine_item,
                                resource,
                                planet,
                                sector,
                                ctx.address(),
                            ));

                            let operation =
                                autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                    stopwatch: timers::Stopwatch::new(),
                                });
                            self.operation = Some(operation);
                        }
                    }
                }
                autoplay::BotOps::Mining(mining_ops) => {
                    mining_ops.timer.tick(msg.0);
                    log::info!("{:#?}", &mining_ops);

                    if mining_ops.timer.finished() {
                        let fleet = self.fleet;
                        let planet = self.planet.0;
                        let mine_item = self.mine_item.0;
                        let mine_item_mint = self.mine_item.1.mint;
                        let resource = self.resource.0;
                        let sector = self.planet.1.sector;

                        self.addr_sage.do_send(SageAction::StopMining(
                            fleet,
                            mine_item,
                            mine_item_mint,
                            resource,
                            planet,
                            sector,
                            ctx.address(),
                        ));

                        let operation = autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                            stopwatch: timers::Stopwatch::new(),
                        });
                        self.operation = Some(operation);
                    }
                }
                autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops) => {
                    starbase_loading_bay_ops.stopwatch.tick(msg.0);
                    log::info!("{:#?}", &starbase_loading_bay_ops);

                    let fleet = self.fleet;
                    match starbase_loading_bay_ops.next_action {
                        autoplay::StarbaseActions::CargoDeposit(cargo_pod_to, mint, amount) => {
                            self.addr_sage.do_send(SageAction::CargoDeposit(
                                fleet,
                                starbase_loading_bay_ops.starbase,
                                cargo_pod_to,
                                mint,
                                amount,
                                ctx.address(),
                            ));

                            let operation =
                                autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                    stopwatch: timers::Stopwatch::new(),
                                });
                            self.operation = Some(operation);
                        }
                        autoplay::StarbaseActions::CargoWithdraw(mint, amount) => {
                            self.addr_sage.do_send(SageAction::CargoWithdraw(
                                fleet,
                                starbase_loading_bay_ops.starbase,
                                mint,
                                amount,
                                ctx.address(),
                            ));

                            let operation =
                                autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                    stopwatch: timers::Stopwatch::new(),
                                });
                            self.operation = Some(operation);
                        }
                        autoplay::StarbaseActions::CheckFuelStatus => {
                            self.addr_sage.do_send(SageRequest::FleetFuelTank(
                                fleet.1.fuel_tank,
                                ctx.address(),
                            ));
                        }
                        autoplay::StarbaseActions::CheckAmmoStatus => {
                            self.addr_sage.do_send(SageRequest::FleetAmmoBank(
                                fleet.1.ammo_bank,
                                ctx.address(),
                            ));
                        }
                        autoplay::StarbaseActions::CheckFoodStatus => {
                            self.addr_sage.do_send(SageRequest::FleetFoodCargoHold(
                                fleet.1.cargo_hold,
                                ctx.address(),
                            ));
                        }
                        autoplay::StarbaseActions::UndockFromStarbase => {
                            self.addr_sage.do_send(SageAction::StarbaseUndock(
                                fleet,
                                starbase_loading_bay_ops.starbase,
                                ctx.address(),
                            ));

                            let operation =
                                autoplay::BotOps::TxsSageBased(autoplay::TxsSageBasedOps {
                                    stopwatch: timers::Stopwatch::new(),
                                });
                            self.operation = Some(operation);
                        }
                    }
                }
            },
            None => {
                // no operation is running requst fleet status
                self.addr_sage
                    .do_send(SageRequest::Fleet(self.fleet.0, ctx.address()));
            }
        }
    }
}
