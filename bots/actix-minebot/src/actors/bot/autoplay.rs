use super::*;

use crate::{
    actors::{Bot, ClockTimeRequest, SageRequest},
    timers,
};

#[derive(Debug)]
pub(crate) enum BotOps {
    Idle(IdleOps),
    Mining(MiningOps),
    StarbaseLoadingBay(StarbaseLoadingBayOps),
    TxsSageBased(TxsSageBasedOps),
}

#[derive(Debug)]
pub(crate) enum IdleActions {
    DockeToStarbase,
    MineAsteroid,
}

#[derive(Debug)]
pub(crate) struct IdleOps {
    pub(crate) sector: [i64; 2],
    cargo_capacity_fraction: f64,
    pub(crate) stop_watch: timers::Stopwatch,
    pub(crate) next_action: IdleActions,
}

#[derive(Debug)]
pub(crate) struct MiningOps {
    mining_location: String,
    currently_mining: String,
    resource_mining_rate_per_second: f32,
    amount_mined: f32,
    pub(crate) timer: timers::Timer,
}

#[derive(Debug)]
pub(crate) struct TxsSageBasedOps {
    pub(crate) stop_watch: timers::Stopwatch,
}

#[derive(Debug, Clone)]
pub(crate) enum StarbaseActions {
    CargoDeposit(Pubkey, Pubkey, u64), // (CargoPodTo, Mint, Amount)
    CargoWithdraw(Pubkey, u64),        // (Mint, Amount)
    CheckFuelStatus,
    CheckAmmoStatus,
    CheckFoodStatus,
    UndockFromStarbase,
}

#[derive(Debug)]
pub(crate) struct StarbaseLoadingBayOps {
    pub(crate) starbase: Pubkey,
    pub(crate) stop_watch: timers::Stopwatch,
    pub(crate) next_action: StarbaseActions,
}

impl Bot {
    pub(crate) fn autoplay_fleet_with_state_update(
        &mut self,
        fleet_with_state: FleetWithState,
        addr: Addr<Bot>,
    ) {
        if self.fleet.is_none() {
            let fleet = fleet_with_state.fleet;
            self.fleet = Some(fleet);
        }

        // let previous = &self.fleet_state;
        // dbg!(previous);

        let fleet_state = fleet_with_state.state;
        match &fleet_state {
            FleetState::Idle(_idle) => {
                if let Some(fleet) = &self.fleet {
                    self.addr_sage
                        .do_send(SageRequest::FleetCargoHold(fleet.cargo_hold.clone(), addr));
                }
            }
            FleetState::MineAsteroid(_mine_asteroid) => {
                match &self.operation {
                    Some(BotOps::Mining(_)) => {} // Do nothing, already performing a mining operation
                    _ => {
                        // Request a "Clock" to kick-off the mining operation
                        self.addr_sage.do_send(ClockTimeRequest(addr));
                    }
                }
            }
            FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                log::info!("{:?}", starbase_loading_bay);

                match &self.operation {
                    Some(BotOps::StarbaseLoadingBay(_)) => {
                        // Do nothing, already performing a starbase loading bay operation
                        // 1. Unload Cargo
                        // 2. Resupply Fuel
                        // 3. Resupply Ammo
                        // 4. Resupply Food
                    }
                    _ => {
                        // Request an update on the fleet's cargo hold to kick-off the starbase loading bay operation
                        if let Some(fleet) = &self.fleet {
                            self.addr_sage.do_send(SageRequest::FleetCargoHold(
                                fleet.cargo_hold.clone(),
                                addr,
                            ));
                        }
                    }
                }
            }
            _ => {}
        }

        self.fleet_state = Some(fleet_state);
    }

    pub(crate) fn autoplay_idle(&self, idle: &Idle) -> IdleOps {
        let fleet = self.fleet.as_ref().expect("Fleet not found");
        let current_capacity = self.fleet_cargo_hold.iter().fold(0, |x, (_, v)| x + v);
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
            stop_watch: timers::Stopwatch::new(),
            next_action,
        }
    }

    pub(crate) fn autoplay_mine_asteroid(
        &self,
        mine_asteroid: &MineAsteroid,
        clock: &Clock,
    ) -> MiningOps {
        let fleet = self.fleet.as_ref().expect("Fleet not found");
        let planet = self.planet.as_ref().expect("Planet not found");
        let resource = self.resource.as_ref().expect("Resource not found");
        let mine_item = self.mine_item.as_ref().expect("MineItem not found");

        let mining_location = planet.name();
        let currently_mining = mine_item.name();

        let mining_rate = calc::asteroid_mining_emission_rate(&fleet.stats, mine_item, resource);

        let cargo_space = fleet.stats.cargo_stats.cargo_capacity;

        let mining_duration = calc::asteroid_mining_resource_extraction_duration(
            &fleet.stats,
            mine_item,
            resource,
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

    pub(crate) fn autoplay_starbase_loading_bay(
        &self,
        starbase_loading_bay: &StarbaseLoadingBay,
    ) -> StarbaseLoadingBayOps {
        let mine_item_mint = &self.mine_args.2;

        let cargo_withdraw_amount = self
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
            StarbaseActions::CargoWithdraw(*mine_item_mint, cargo_withdraw_amount)
        } else {
            match &self.operation {
                Some(BotOps::StarbaseLoadingBay(starbase_loading_bay_ops)) => {
                    match starbase_loading_bay_ops.next_action {
                        StarbaseActions::CheckFuelStatus => {
                            let fleet = self.fleet.as_ref().expect("Fleet not found");
                            let (fuel_mint, fuel_amount) = &self.fleet_fuel_tank[0];
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
                            let fleet = self.fleet.as_ref().expect("Fleet not found");
                            let (ammo_mint, ammo_amount) = &self.fleet_ammo_bank[0];
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
                            let fleet = self.fleet.as_ref().expect("Fleet not found");
                            let (food_mint, food_amount) = &self.fleet_food_cargo[0];
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

        let stop_watch = match &self.operation {
            Some(BotOps::StarbaseLoadingBay(starbase_loading_bay_ops)) => {
                starbase_loading_bay_ops.stop_watch.clone()
            }
            _ => timers::Stopwatch::new(),
        };

        StarbaseLoadingBayOps {
            starbase: starbase_loading_bay.starbase.clone(),
            stop_watch,
            next_action,
        }
    }
}
