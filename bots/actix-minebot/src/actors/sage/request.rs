use super::*;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub enum SageRequest {
    Fleet(Pubkey, Addr<BotActor>),              // (Fleet, Callback)
    FleetAmmoBank(Pubkey, Addr<BotActor>),      // (Fleet's Ammo Bank, Callback
    FleetCargoHold(Pubkey, Addr<BotActor>),     // (Fleet's Cargo Hold, Callback)
    FleetFuelTank(Pubkey, Addr<BotActor>),      // (Fleet's Fuel Tank, Callback)
    FleetFoodCargoHold(Pubkey, Addr<BotActor>), // (Fleet's Cargo Hold, Callback)
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum SageResponse {
    Fleet(FleetWithState),
    FleetAmmoBank(Vec<(String, u64)>),
    FleetCargoHold(Vec<(String, u64)>),
    FleetFuelTank(Vec<(String, u64)>),
    FleetFoodCargoHold(Vec<(String, u64)>),
}

impl Handler<SageRequest> for SageBasedActor {
    type Result = ();

    fn handle(&mut self, msg: SageRequest, ctx: &mut Context<Self>) -> Self::Result {
        let program = self.client.program(SAGE_ID).unwrap();

        match msg {
            SageRequest::Fleet(fleet_id, addr_bot) => {
                let fut = async move {
                    match SageBasedGameHandler::get_fleet_with_state(&program, &fleet_id).await {
                        Ok((_, fleet_with_state)) => {
                            addr_bot.do_send(SageResponse::Fleet(fleet_with_state));
                        }
                        Err(err) => {
                            log::error!("{:?}", &err);
                            addr_bot.do_send(Ping(None));
                        }
                    }
                };

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageRequest::FleetAmmoBank(ammo_bank, addr_bot) => {
                let rpc = program.async_rpc();

                let fut = Box::pin(async move {
                    let token_accounts =
                        SageBasedGameHandler::parsed_token_account_amounts(&rpc, &ammo_bank).await;
                    addr_bot.do_send(SageResponse::FleetAmmoBank(token_accounts));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageRequest::FleetCargoHold(cargo_hold, addr_bot) => {
                let rpc = program.async_rpc();

                let fut = Box::pin(async move {
                    let token_accounts =
                        SageBasedGameHandler::parsed_token_account_amounts(&rpc, &cargo_hold).await;
                    addr_bot.do_send(SageResponse::FleetCargoHold(token_accounts));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageRequest::FleetFuelTank(fuel_tank, addr_bot) => {
                let rpc = program.async_rpc();

                let fut = Box::pin(async move {
                    let token_accounts =
                        SageBasedGameHandler::parsed_token_account_amounts(&rpc, &fuel_tank).await;
                    addr_bot.do_send(SageResponse::FleetFuelTank(token_accounts));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
            SageRequest::FleetFoodCargoHold(cargo_hold, addr_bot) => {
                let rpc = program.async_rpc();
                let game = self.game.clone();

                let fut = Box::pin(async move {
                    let token_accounts =
                        SageBasedGameHandler::parsed_token_account_amounts(&rpc, &cargo_hold).await;

                    let food_token_accounts: Vec<(String, u64)> = token_accounts
                        .into_iter()
                        .filter(|(mint, _amount)| mint == &game.mints.food.to_string())
                        .collect();

                    addr_bot.do_send(SageResponse::FleetFoodCargoHold(food_token_accounts));
                });

                let actor_future = fut.into_actor(self);

                ctx.spawn(actor_future);
            }
        }
    }
}
