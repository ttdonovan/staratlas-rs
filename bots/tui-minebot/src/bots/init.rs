use super::*;

fn ui_masked_pubkey(pubkey: &Pubkey) -> String {
    let id = pubkey.to_string();
    format!("{}...{}", &id[..4], &id[40..])
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
                            ui_masked_pubkey(&fleet_pubkey),
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
                        txs_counter: 0,
                        txs_errors: 0,
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
                        ui_masked_pubkey(&fleet_pubkey),
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
                    txs_counter: 0,
                    txs_errors: 0,
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
                            ui_masked_pubkey(&fleet_pubkey),
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
                        txs_counter: 0,
                        txs_errors: 0,
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
