use super::*;

impl Handler<SageResponse> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: SageResponse, ctx: &mut Context<Self>) {
        match msg {
            SageResponse::Fleet(fleet_with_state) => {
                self.operation = None;
                self.autoplay_fleet_with_state_update(fleet_with_state, ctx.address());
            }
            SageResponse::FleetAmmoBank(ammo_bank) => {
                self.fleet_ammo_bank = ammo_bank;

                match &self.fleet_state {
                    Some(FleetState::StarbaseLoadingBay(starbase_loading_bay)) => {
                        let starbase_loading_bay_ops =
                            self.autoplay_starbase_loading_bay(&starbase_loading_bay);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        self.operation = Some(operation);
                    }
                    _ => {}
                }
            }
            SageResponse::FleetCargoHold(cargo_hold) => {
                self.fleet_cargo_hold = cargo_hold;

                match &self.fleet_state {
                    Some(FleetState::Idle(idle)) => {
                        let idle_ops = self.autoplay_idle(&idle);
                        let operation = autoplay::BotOps::Idle(idle_ops);

                        self.operation = Some(operation);
                    }
                    Some(FleetState::StarbaseLoadingBay(starbase_loading_bay)) => {
                        let starbase_loading_bay_ops =
                            self.autoplay_starbase_loading_bay(&starbase_loading_bay);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        self.operation = Some(operation);
                    }
                    _ => {}
                }
            }
            SageResponse::FleetFuelTank(fuel_tank) => {
                self.fleet_fuel_tank = fuel_tank;

                match &self.fleet_state {
                    Some(FleetState::StarbaseLoadingBay(starbase_loading_bay)) => {
                        let starbase_loading_bay_ops =
                            self.autoplay_starbase_loading_bay(&starbase_loading_bay);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        self.operation = Some(operation);
                    }
                    _ => {}
                }
            }
            SageResponse::FleetFoodCargoHold(food_cargo) => {
                self.fleet_food_cargo = food_cargo;

                match &self.fleet_state {
                    Some(FleetState::StarbaseLoadingBay(starbase_loading_bay)) => {
                        let starbase_loading_bay_ops =
                            self.autoplay_starbase_loading_bay(&starbase_loading_bay);
                        let operation =
                            autoplay::BotOps::StarbaseLoadingBay(starbase_loading_bay_ops);

                        self.operation = Some(operation);
                    }
                    _ => {}
                }
            }
        }
    }
}
