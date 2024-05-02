use super::*;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum SageAction {
    CargoDeposit((Pubkey, Fleet), Pubkey, Pubkey, Pubkey, u64, Addr<Bot>), // ((FleetId, Fleet), Starbase, CargoPodTo, Mint, Amount, Addr<Bot>)
    CargoWithdraw((Pubkey, Fleet), Pubkey, Pubkey, u64, Addr<Bot>), // ((FleetId, Fleet), Starbase, Mint, Amount, Addr<Bot>)
    StarbaseDock((Pubkey, Fleet), [i64; 2], Addr<Bot>), // ((FleetId, Fleet), Sector, Addr<Bot>)
    StarbaseUndock((Pubkey, Fleet), Pubkey, Addr<Bot>), // ((FleetId, Fleet), Starbase, Addr<Bot>)
    StartMining((Pubkey, Fleet), Pubkey, Pubkey, Pubkey, [i64; 2], Addr<Bot>), // ((FleetId, Fleet), MineItem, Resource, Planet, Sector, Addr<Bot>)
    StopMining(
        (Pubkey, Fleet),
        Pubkey,
        Pubkey,
        Pubkey,
        Pubkey,
        [i64; 2],
        Addr<Bot>,
    ), // ((FleetId, Fleet), MineItem, MineItemMint, Resource, Planet, Sector, Addr<Bot>)
}

impl Handler<SageAction> for SageBased {
    type Result = ();

    fn handle(&mut self, msg: SageAction, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            SageAction::CargoDeposit(fleet, starbase, cargo_pod_to, mint, amount, addr_bot) => {
                let sage_program = self.client.program(SAGE_ID).unwrap();
                let cargo_program = self.client.program(CARGO_ID).unwrap();
                let payer = self.payer.clone();

                let game_id = self.game_id.clone();
                let game = self.game.clone();

                let (fleet_id, fleet) = fleet;

                let fut = Box::pin(async move {
                    let result = SageBasedGameHandler::cargo_deposit_to_fleet(
                        &sage_program,
                        &cargo_program,
                        &payer,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        &starbase,
                        &cargo_pod_to,
                        &mint,
                        amount,
                    )
                    .await;

                    let signature = match result {
                        Some(Ok(signature)) => Some(signature),
                        Some(Err(err)) => {
                            log::error!("{:?}", &err);
                            None
                        }
                        None => {
                            log::error!("Simulation failed?");
                            None
                        }
                    };

                    addr_bot.do_send(Ping(signature));
                });

                let actor_future = fut.into_actor(self);
                ctx.spawn(actor_future);
            }
            SageAction::CargoWithdraw(fleet, starbase, mint, amount, addr_bot) => {
                let sage_program = self.client.program(SAGE_ID).unwrap();
                let cargo_program = self.client.program(CARGO_ID).unwrap();
                let payer = self.payer.clone();

                let game_id = self.game_id.clone();
                let game = self.game.clone();

                let (fleet_id, fleet) = fleet;

                let fut = Box::pin(async move {
                    let result = SageBasedGameHandler::cargo_withdraw_from_fleet(
                        &sage_program,
                        &cargo_program,
                        &payer,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        &starbase,
                        &mint,
                        amount,
                    )
                    .await;

                    let signature = match result {
                        Some(Ok(signature)) => Some(signature),
                        Some(Err(err)) => {
                            log::error!("{:?}", &err);
                            None
                        }
                        None => {
                            log::error!("Simulation failed?");
                            None
                        }
                    };

                    addr_bot.do_send(Ping(signature));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageAction::StarbaseDock(fleet, sector, addr_bot) => {
                let program = self.client.program(SAGE_ID).unwrap();
                let payer = self.payer.clone();

                let game_id = self.game_id.clone();
                let game = self.game.clone();

                let (fleet_id, fleet) = fleet;

                let fut = Box::pin(async move {
                    let result = SageBasedGameHandler::dock_to_starbase(
                        &program,
                        &payer,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        sector,
                    )
                    .await;

                    let signature = match result {
                        Some(Ok(signature)) => Some(signature),
                        Some(Err(err)) => {
                            log::error!("{:?}", &err);
                            None
                        }
                        None => {
                            log::error!("Simulation failed?");
                            None
                        }
                    };

                    addr_bot.do_send(Ping(signature));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageAction::StarbaseUndock(fleet, starbase, addr_bot) => {
                let program = self.client.program(SAGE_ID).unwrap();
                let payer = self.payer.clone();

                let game_id = self.game_id.clone();
                let game = self.game.clone();

                let (fleet_id, fleet) = fleet;

                let fut = Box::pin(async move {
                    let result = SageBasedGameHandler::undock_from_starbase(
                        &program,
                        &payer,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        &starbase,
                    )
                    .await;

                    let signature = match result {
                        Some(Ok(signature)) => Some(signature),
                        Some(Err(err)) => {
                            log::error!("{:?}", &err);
                            None
                        }
                        None => {
                            log::error!("Simulation failed?");
                            None
                        }
                    };

                    addr_bot.do_send(Ping(signature));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageAction::StartMining(fleet, mine_item, resource, planet, sector, addr_bot) => {
                let program = self.client.program(SAGE_ID).unwrap();
                let payer = self.payer.clone();

                let game_id = self.game_id.clone();
                let game = self.game.clone();

                let (fleet_id, fleet) = fleet;

                let fut = Box::pin(async move {
                    let result = SageBasedGameHandler::start_mining(
                        &program,
                        &payer,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        &mine_item,
                        &resource,
                        &planet,
                        sector,
                    )
                    .await;

                    let signature = match result {
                        Some(Ok(signature)) => Some(signature),
                        Some(Err(err)) => {
                            log::error!("{:?}", &err);
                            None
                        }
                        None => {
                            log::error!("Simulation failed?");
                            None
                        }
                    };

                    addr_bot.do_send(Ping(signature));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageAction::StopMining(
                fleet,
                mine_item,
                mine_item_mint,
                resource,
                planet,
                sector,
                addr_bot,
            ) => {
                let program = self.client.program(SAGE_ID).unwrap();
                let payer = self.payer.clone();

                let game_id = self.game_id.clone();
                let game = self.game.clone();

                let (fleet_id, fleet) = fleet;

                let fut = Box::pin(async move {
                    let result = SageBasedGameHandler::stop_mining(
                        &program,
                        &payer,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        &mine_item,
                        &mine_item_mint,
                        &resource,
                        &planet,
                        sector,
                    )
                    .await;

                    let signature = match result {
                        Some(Ok(signature)) => Some(signature),
                        Some(Err(err)) => {
                            log::error!("{:?}", &err);
                            None
                        }
                        None => {
                            log::error!("Simulation failed?");
                            None
                        }
                    };

                    addr_bot.do_send(Ping(signature));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
        }
    }
}
