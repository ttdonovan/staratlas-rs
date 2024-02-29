use anchor_client::solana_sdk::{pubkey::Pubkey, signature};

use std::time::Duration;

use crate::{
    sage::{self, FleetState},
    time,
};

fn masked_pubkey(pubkey: &Pubkey) -> String {
    let id = pubkey.to_string();
    format!("{}...{}", &id[..4], &id[40..])
}

fn calc_asteroid_mining_emission_rate(
    fleet: &sage::Fleet,
    resource: &sage::Resource,
    mine_item: &sage::MineItem,
) -> f32 {
    let mining_rate = fleet.0.stats.cargo_stats.mining_rate as f32;
    let system_richness = resource.0.system_richness as f32;
    let resource_harndess = mine_item.0.resource_hardness as f32;

    (mining_rate / 10000.0) * (system_richness / resource_harndess)
}

fn calc_asteroid_mining_amount(
    cargo_amount: u32,
    cargo_capacity: u32,
    emission_rate: f32,
    mine_asteroid_start: i64,
) -> u32 {
    let mining_time_elapsed = if mine_asteroid_start == 0 {
        0
    } else {
        time::get_time() as i64 - mine_asteroid_start
    };
    let est_amount_minded = mining_time_elapsed as f32 * emission_rate;
    let mut est_cargo_amount = cargo_amount + est_amount_minded as u32;
    est_cargo_amount = est_cargo_amount.min(cargo_capacity);
    cargo_capacity - est_cargo_amount
}

fn calc_asteroid_mining_duration(amount: u32, emission_rate: f32) -> Duration {
    Duration::from_secs_f32(amount as f32 / emission_rate)
}

pub fn init_bots(
    sage: &sage::SageContext,
    fleet_ids: Vec<Pubkey>,
) -> anyhow::Result<Vec<MiningBot>> {
    let mut bots = vec![];

    for fleet_pubkey in fleet_ids {
        let (fleet, fleet_state) = sage.fleet_with_state_accts(&fleet_pubkey)?;

        let cargo_stats = &fleet.0.stats.cargo_stats;
        let fuel_tank: FuelTank = (fleet.0.fuel_tank, 0, cargo_stats.fuel_capacity);
        let ammo_bank: AmmoBank = (fleet.0.ammo_bank, 0, cargo_stats.ammo_capacity);
        let cargo_hold: CargoHold = (fleet.0.cargo_hold, 0, cargo_stats.cargo_capacity);

        match &fleet_state {
            FleetState::Idle(idle) => {
                let planets = sage.planet_accts(idle.sector)?;
                if let Some((planet_pubkey, _planet)) = planets
                    .iter()
                    .find(|(_, planet)| planet.0.num_resources == 1)
                {
                    let mut resources = sage.resource_accts(planet_pubkey)?;
                    let (resource_pubkey, resource) = resources.remove(0);
                    let (mine_item_pubkey, mine_item) =
                        sage.mine_item_acct(&resource.0.mine_item)?;

                    let mine_asteroid_emission_rate =
                        calc_asteroid_mining_emission_rate(&fleet, &resource, &mine_item);
                    let mine_asteroid_start = 0;

                    bots.push(MiningBot {
                        fleet: (
                            fleet_pubkey,
                            masked_pubkey(&fleet_pubkey),
                            fleet.fleet_label().to_string(),
                            fleet,
                            fleet_state,
                        ),
                        is_fleet_state_dirty: false,
                        fuel_tank,
                        ammo_bank,
                        cargo_hold,
                        resource: (resource_pubkey, resource),
                        mine_item: (mine_item_pubkey, mine_item.name().to_string(), mine_item),
                        mine_asteroid_emission_rate,
                        mine_asteroid_start,
                        mine_asteroid_amount: 0,
                        mine_asteroid_duraiton: Duration::ZERO,
                        mining_timer: time::Timer::default(),
                        autoplay: Autoplay::IsIdle,
                        txs: None,
                    });
                }
            }
            FleetState::MineAsteroid(mine_asteroid) => {
                let mut resources = sage.resource_accts(&mine_asteroid.asteroid)?;
                let (resource_pubkey, resource) = resources.remove(0);
                let (mine_item_pubkey, mine_item) = sage.mine_item_acct(&resource.0.mine_item)?;

                let mine_asteroid_emission_rate =
                    calc_asteroid_mining_emission_rate(&fleet, &resource, &mine_item);
                let mine_asteroid_start = mine_asteroid.start;

                bots.push(MiningBot {
                    fleet: (
                        fleet_pubkey,
                        masked_pubkey(&fleet_pubkey),
                        fleet.fleet_label().to_string(),
                        fleet,
                        fleet_state,
                    ),
                    is_fleet_state_dirty: false,
                    fuel_tank,
                    ammo_bank,
                    cargo_hold,
                    resource: (resource_pubkey, resource),
                    mine_item: (mine_item_pubkey, mine_item.name().to_string(), mine_item),
                    mine_asteroid_emission_rate,
                    mine_asteroid_start,
                    mine_asteroid_amount: 0,
                    mine_asteroid_duraiton: Duration::ZERO,
                    mining_timer: time::Timer::default(),
                    autoplay: Autoplay::IsMiningAsteroid,
                    txs: None,
                });
            }
            FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                let (_, starbase) = sage.starbase_acct(&starbase_loading_bay.starbase)?;
                let planets = sage.planet_accts(starbase.0.sector)?;
                if let Some((planet_pubkey, _planet)) = planets
                    .iter()
                    .find(|(_, planet)| planet.0.num_resources == 1)
                {
                    let mut resources = sage.resource_accts(planet_pubkey)?;
                    let (resource_pubkey, resource) = resources.remove(0);
                    let (mine_item_pubkey, mine_item) =
                        sage.mine_item_acct(&resource.0.mine_item)?;

                    let mine_asteroid_emission_rate =
                        calc_asteroid_mining_emission_rate(&fleet, &resource, &mine_item);
                    let mine_asteroid_start = 0;

                    bots.push(MiningBot {
                        fleet: (
                            fleet_pubkey,
                            masked_pubkey(&fleet_pubkey),
                            fleet.fleet_label().to_string(),
                            fleet,
                            fleet_state,
                        ),
                        is_fleet_state_dirty: false,
                        fuel_tank,
                        ammo_bank,
                        cargo_hold,
                        resource: (resource_pubkey, resource),
                        mine_item: (mine_item_pubkey, mine_item.name().to_string(), mine_item),
                        mine_asteroid_emission_rate,
                        mine_asteroid_start,
                        mine_asteroid_amount: 0,
                        mine_asteroid_duraiton: Duration::ZERO,
                        mining_timer: time::Timer::default(),
                        autoplay: Autoplay::StarbaseHangarCargoWithdraw,
                        txs: None,
                    });
                }
            }
            _ => {}
        }
    }

    for bot in bots.iter_mut() {
        let fuel_amount = sage.get_token_account_balances_by_owner(&bot.fuel_tank.0)?;
        bot.set_fuel_amount(fuel_amount);

        let ammo_amount = sage.get_token_account_balances_by_owner(&bot.ammo_bank.0)?;
        bot.set_ammo_amount(ammo_amount);

        let cargo_amount = sage.get_token_account_balances_by_owner(&bot.cargo_hold.0)?;
        bot.set_cargo_amount(cargo_amount);

        // re-calculate mining amount and duration
        bot.mine_asteroid_amount = calc_asteroid_mining_amount(
            bot.cargo_hold.1,
            bot.cargo_hold.2,
            bot.mine_asteroid_emission_rate,
            bot.mine_asteroid_start,
        );
        bot.mine_asteroid_duraiton = calc_asteroid_mining_duration(
            bot.mine_asteroid_amount,
            bot.mine_asteroid_emission_rate,
        );

        if bot.autoplay == Autoplay::IsMiningAsteroid {
            bot.mining_timer.set_duration(bot.mine_asteroid_duraiton);
        }
    }

    Ok(bots)
}

pub fn run_update(
    bot: &mut MiningBot,
    dt: Duration,
    sage: &sage::SageContext,
) -> anyhow::Result<()> {
    bot.tick(dt);

    if bot.is_fleet_state_dirty {
        let (_, fleet_state) = sage.fleet_with_state_accts(&bot.fleet.0)?;
        bot.fleet.4 = fleet_state;
        bot.is_fleet_state_dirty = false;

        match &bot.autoplay {
            Autoplay::IsMiningAsteroid => {
                // re-calculate fuel, ammo, cargo, mining amount/duration and set mining timer
                let fuel_amount = sage.get_token_account_balances_by_owner(&bot.fuel_tank.0)?;
                bot.set_fuel_amount(fuel_amount);

                let ammo_amount = sage.get_token_account_balances_by_owner(&bot.ammo_bank.0)?;
                bot.set_ammo_amount(ammo_amount);

                let cargo_amount = sage.get_token_account_balances_by_owner(&bot.cargo_hold.0)?;
                bot.set_cargo_amount(cargo_amount);

                bot.mine_asteroid_amount = calc_asteroid_mining_amount(
                    bot.cargo_hold.1,
                    bot.cargo_hold.2,
                    bot.mine_asteroid_emission_rate,
                    bot.mine_asteroid_start,
                );

                bot.mine_asteroid_duraiton = calc_asteroid_mining_duration(
                    bot.mine_asteroid_amount,
                    bot.mine_asteroid_emission_rate,
                );

                bot.mining_timer.set_duration(bot.mine_asteroid_duraiton);
                bot.mining_timer.reset();
            }
            Autoplay::StarbaseHangerCargoDeposit(deposit) => match deposit {
                CargoDeposit::Fuel => {
                    let fuel_amount = sage.get_token_account_balances_by_owner(&bot.fuel_tank.0)?;
                    bot.set_fuel_amount(fuel_amount);
                }
                CargoDeposit::Ammo => {
                    let ammo_amount = sage.get_token_account_balances_by_owner(&bot.ammo_bank.0)?;
                    bot.set_ammo_amount(ammo_amount);
                }
                CargoDeposit::Food => {
                    let cargo_amount =
                        sage.get_token_account_balances_by_owner(&bot.cargo_hold.0)?;
                    bot.set_cargo_amount(cargo_amount);
                }
            },
            _ => {}
        }

        if bot.autoplay == Autoplay::IsMiningAsteroid {}
    }

    match &bot.autoplay {
        Autoplay::IsIdle => {
            // FIXME: need to check for enough food to start mining...
            // can fake it for now
            let (_, actual, capacity) = &bot.cargo_hold;
            if (*actual as f32 / *capacity as f32) <= 0.5 {
                bot.autoplay = Autoplay::StartMiningAsteroid;
            } else {
                bot.autoplay = Autoplay::StarbaseDock;
            }
        }
        Autoplay::StartMiningAsteroid => {
            match sage.start_mining_asteroid(&bot.fleet.0, &bot.fleet.3, &bot.fleet.4) {
                Ok(signature) => {
                    bot.autoplay = Autoplay::IsMiningAsteroid;
                    bot.is_fleet_state_dirty = true;
                    bot.txs = Some(signature);
                }
                Err(_err) => {
                    // println!("Error: {:?}", err);
                }
            }
        }
        Autoplay::IsMiningAsteroid => {
            if bot.mining_timer.finished() {
                bot.autoplay = Autoplay::StopMiningAsteroid;
            }
            bot.is_fleet_state_dirty = false;
        }
        Autoplay::StopMiningAsteroid => {
            match sage.stop_mining_asteroid(&bot.fleet.0, &bot.fleet.3, &bot.fleet.4) {
                Ok(signature) => {
                    bot.autoplay = Autoplay::StarbaseDock;
                    bot.is_fleet_state_dirty = true;
                    bot.txs = Some(signature);
                }
                Err(_err) => {
                    // println!("Error: {:?}", err);
                }
            }
        }
        Autoplay::StarbaseDock => {
            let (fleet_id, _, _, fleet, state) = &bot.fleet;

            match sage.dock_to_starbase(fleet_id, fleet, state) {
                Ok(signature) => {
                    bot.autoplay = Autoplay::StarbaseHangarCargoWithdraw;
                    bot.is_fleet_state_dirty = true;
                    bot.txs = Some(signature);
                }
                Err(_err) => {
                    // println!("Error: {:?}", err);
                }
            }
        }
        Autoplay::StarbaseUndock => {
            let (fleet_id, _, _, fleet, state) = &bot.fleet;

            match sage.undock_from_starbase(fleet_id, fleet, state) {
                Ok(signature) => {
                    bot.autoplay = Autoplay::IsIdle;
                    bot.is_fleet_state_dirty = true;
                    bot.txs = Some(signature);
                }
                Err(_err) => {
                    // println!("Error: {:?}", err);
                }
            }
        }
        Autoplay::StarbaseHangarCargoWithdraw => {
            if let Some(starbase) = bot.starbase() {
                let fleet_id = &bot.fleet.0;
                let fleet = &bot.fleet.3;
                let mine_item = &bot.mine_item.2;

                match sage.withdraw_from_fleet(
                    fleet_id,
                    fleet,
                    starbase,
                    &mine_item.0.mint,
                    u64::MAX,
                ) {
                    Ok(signature) => {
                        if let Some(signature) = signature {
                            bot.txs = Some(signature);
                        }

                        bot.is_fleet_state_dirty = true;
                        bot.autoplay = Autoplay::StarbaseHangerCargoDeposit(CargoDeposit::Fuel);
                    }
                    Err(_err) => {
                        // println!("Error: {:?}", err);
                    }
                }
            } else {
                bot.is_fleet_state_dirty = true;
            }
        }
        Autoplay::StarbaseHangerCargoDeposit(deposite) => {
            if let Some(starbase) = bot.starbase() {
                let fleet_id = &bot.fleet.0;
                let fleet = &bot.fleet.3;

                let resupply_and_next_deposit = match deposite {
                    CargoDeposit::Fuel => {
                        // Fuel tank refuel
                        let (fuel_tank, actual, capacity) = &bot.fuel_tank;
                        let fuel_mint = &sage.game_acct.0.mints.fuel;

                        let amount = (capacity - actual) as u64;
                        let next_deposit = Some(CargoDeposit::Ammo);

                        if (*actual as f32 / *capacity as f32) < 0.5 {
                            let resupply = Some((fuel_tank, fuel_mint, amount));

                            Some((resupply, next_deposit))
                        } else {
                            Some((None, next_deposit))
                        }
                    }
                    CargoDeposit::Ammo => {
                        // Ammo bank rearm
                        let (ammo_bank, actual, capacity) = &bot.ammo_bank;
                        let ammo_mint = &sage.game_acct.0.mints.ammo;

                        let amount = (capacity - actual) as u64;
                        let next_deposit = Some(CargoDeposit::Food);

                        if (*actual as f32 / *capacity as f32) < 0.5 {
                            let resupply = Some((ammo_bank, ammo_mint, amount));

                            Some((resupply, next_deposit))
                        } else {
                            Some((None, next_deposit))
                        }
                    }
                    CargoDeposit::Food => {
                        // Cargo hold supply (food)
                        let (cargo_hold, actual, capacity) = &bot.cargo_hold;
                        let food_mint = &sage.game_acct.0.mints.food;

                        let amount = (*capacity as f32 * 0.05) as u64;
                        let next_deposit = None;

                        if (*actual as f32 / *capacity as f32) < 0.05 {
                            let resupply = Some((cargo_hold, food_mint, amount));

                            Some((resupply, next_deposit))
                        } else {
                            Some((None, next_deposit))
                        }
                    }
                };

                match resupply_and_next_deposit {
                    Some((resupply, next_action)) => {
                        match resupply {
                            Some((cargo_pod_to, mint, amount)) => {
                                match sage.deposit_to_fleet(
                                    fleet_id,
                                    fleet,
                                    starbase,
                                    cargo_pod_to,
                                    mint,
                                    amount,
                                ) {
                                    Ok(signature) => {
                                        bot.txs = Some(signature);
                                        bot.is_fleet_state_dirty = true;

                                        match next_action {
                                            Some(next_action) => {
                                                bot.autoplay = Autoplay::StarbaseHangerCargoDeposit(
                                                    next_action,
                                                );
                                            }
                                            None => {
                                                bot.autoplay = Autoplay::StarbaseUndock;
                                            }
                                        }
                                    }
                                    Err(_err) => {
                                        // println!("Error: {:?}", err);
                                    }
                                }
                            }
                            None => match next_action {
                                Some(next_action) => {
                                    bot.is_fleet_state_dirty = true;
                                    bot.autoplay =
                                        Autoplay::StarbaseHangerCargoDeposit(next_action);
                                }
                                None => {
                                    bot.autoplay = Autoplay::StarbaseUndock;
                                }
                            },
                        }
                    }
                    None => {
                        bot.autoplay = Autoplay::StarbaseUndock;
                    }
                }
            } else {
                bot.is_fleet_state_dirty = true;
            }
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Autoplay {
    IsIdle,
    StartMiningAsteroid,
    IsMiningAsteroid,
    StopMiningAsteroid,
    StarbaseDock,
    StarbaseUndock,
    StarbaseHangarCargoWithdraw,
    StarbaseHangerCargoDeposit(CargoDeposit),
}

#[derive(Debug, PartialEq)]
pub enum CargoDeposit {
    Fuel,
    Ammo,
    Food,
}

type Fleet = (Pubkey, String, String, sage::Fleet, sage::FleetState); // (Pubkey, mask, callsign, fleet, fleet_state)
type FuelTank = (Pubkey, u32, u32); // (Pubkey, amount, capacity)
type AmmoBank = (Pubkey, u32, u32); // (Pubkey, amount, capacity)
type CargoHold = (Pubkey, u32, u32); // (Pubkey, amount, capacity)
type Resource = (Pubkey, sage::Resource);
type MineItem = (Pubkey, String, sage::MineItem);

#[derive(Debug)]
pub struct MiningBot {
    pub fleet: Fleet,
    pub is_fleet_state_dirty: bool,
    pub fuel_tank: FuelTank,
    pub ammo_bank: AmmoBank,
    pub cargo_hold: CargoHold,
    pub resource: Resource,
    pub mine_item: MineItem,
    pub mine_asteroid_emission_rate: f32,
    pub mine_asteroid_start: i64,
    pub mine_asteroid_amount: u32,
    pub mine_asteroid_duraiton: Duration,
    pub mining_timer: time::Timer,
    pub autoplay: Autoplay,
    pub txs: Option<signature::Signature>,
}

impl MiningBot {
    pub fn fleet_id(&self) -> &Pubkey {
        &self.fleet.0
    }

    pub fn masked_fleet_id(&self) -> &str {
        &self.fleet.1
    }

    pub fn fleet_name(&self) -> &str {
        &self.fleet.2
    }

    pub fn mine_item_name(&self) -> &str {
        &self.mine_item.1
    }

    pub fn mine_rate(&self) -> f32 {
        self.mine_asteroid_emission_rate
    }

    pub fn mine_amount(&self) -> u32 {
        self.mine_asteroid_amount
    }

    pub fn mine_duration(&self) -> Duration {
        self.mine_asteroid_duraiton
    }

    pub fn last_txs(&self) -> String {
        match self.txs.as_ref() {
            Some(signature) => signature.to_string(),
            None => String::from(""),
        }
    }

    pub fn set_fuel_amount(&mut self, amount: u32) {
        let amount = amount.min(self.fuel_tank.2);
        self.fuel_tank.1 = amount;
    }

    pub fn set_ammo_amount(&mut self, amount: u32) {
        let amount = amount.min(self.ammo_bank.2);
        self.ammo_bank.1 = amount;
    }

    pub fn set_cargo_amount(&mut self, amount: u32) {
        let amount = amount.min(self.cargo_hold.2);
        self.cargo_hold.1 = amount;
    }

    pub fn starbase(&self) -> Option<&Pubkey> {
        match &self.fleet.4 {
            sage::FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                Some(&starbase_loading_bay.starbase)
            }
            _ => None,
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        self.mining_timer.tick(dt);
    }
}
