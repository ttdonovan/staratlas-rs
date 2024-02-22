use macroquad::prelude::*;
use spl_associated_token_account::get_associated_token_address;

use staratlas_sage_sdk::{derive, FleetState};

use crate::{bot, sage, ui};
use bot::Autoplay;

struct BotStats {
    is_mining_state: bool,
    emission_rate: f32,
    resource_amount: f32,
    mining_duration: f32,
    mining_end_time: f64,
    mining_countdown: f64,
}

impl Default for BotStats {
    fn default() -> Self {
        BotStats {
            is_mining_state: false,
            emission_rate: 0.0,
            resource_amount: 0.0,
            mining_duration: 0.0,
            mining_end_time: 0.0,
            mining_countdown: 0.0,
        }
    }
}

pub struct App {
    bots: Vec<(bot::Bot, BotStats)>,
    resource_counter: u64,
}

impl App {
    pub async fn run(&mut self, game_handler: &sage::GameHandler) -> anyhow::Result<()> {
        let rpc_client = game_handler.sage_program.rpc();

        loop {
            clear_background(BLUE);
            let time = get_time();

            ui::print_lines(
                vec![
                    &format!("Resource Counter: {}", &self.resource_counter),
                    &format!("Elapsed Time: {:2}", time),
                    &format!("-----------------------------"),
                ],
                0.0,
            );

            for (pos, (bot, stats)) in &mut self.bots.iter_mut().enumerate() {
                let y_offset = 60.0 + (pos * 140) as f32;

                ui::print_lines(
                    vec![
                        &format!("Fleet ID: {}", &bot.fleet_id),
                        &format!("Fleet State: {:?}", &bot.fleet_state),
                        &format!("Is Mining?: {:?}", &stats.is_mining_state),
                        &format!(
                            "Mining: Rate({:2}) Amount({})",
                            &stats.emission_rate, &stats.resource_amount
                        ),
                        &format!("Countdown: {:?}", &stats.mining_countdown),
                        &format!("Autoplay ({:?}): {:?}", &bot.autoplay, &bot.next_action),
                        &format!("-----------------------------"),
                    ],
                    y_offset,
                );

                if bot.fleet_state_dirty {
                    // 1. Refresh fleet state (if dirty)
                    let (_, fleet_state) = derive::fleet_account_with_state(
                        &game_handler.sage_program,
                        &bot.fleet_id,
                    )?;

                    bot.set_fleet_sate(fleet_state);
                }

                match bot.fleet_state {
                    FleetState::Idle(idle) => match idle.sector {
                        [-40, 30] | [0, -39] | [40, 30] => {
                            ui::print_input(
                                "Dock to Starbase (Mouse Left) | Mine Asteroid (Mouse Right)",
                            );

                            if is_mouse_button_pressed(MouseButton::Left)
                                || bot.is_autoplay(Autoplay::ReadyStarbaseDock)
                            {
                                info!("Prepare to dock to starbase");

                                if let Ok(signature) = game_handler.dock_to_starbase(bot) {
                                    info!("Dock Signature: {:?}", signature);

                                    bot.fleet_state_dirty = true;
                                    bot.set_next_action(Autoplay::ManageHangarCargo);
                                }
                            }

                            if is_mouse_button_pressed(MouseButton::Right)
                                || bot.is_autoplay(Autoplay::StartMiningAsteroid)
                            {
                                info!("Prepare to mine asteroid");

                                if let Ok(signature) = game_handler.start_mining_asteroid(bot) {
                                    info!("Mining Start Signature: {:?}", signature);

                                    bot.fleet_state_dirty = true;
                                    bot.set_next_action(Autoplay::IsMiningAstroid);
                                }
                            }
                        }
                        _ => unimplemented!(),
                    },
                    FleetState::MineAsteroid(_mine_asteroid) => {
                        ui::print_input("Stop Mine Asteroid (Mouse Left)");

                        if !stats.is_mining_state {
                            // // mine asteroid's resource (account) and mine item (account)
                            // let resource =
                            //     derive_account::<_, state::Resource>(&game_handler.sage_program, &mine_asteroid.resource)?;
                            // let mine_item =
                            //     derive_account::<_, state::MineItem>(&game_handler.sage_program, &resource.mine_item)?;

                            // dbg!(mine_item.resource_hardness);
                            // dbg!(resource.system_richness);
                            let resource_hardness: f32 = 100.0 / 100.0;
                            let system_richness: f32 = 100.0 / 100.0;

                            // calculate asteroid mining emission rate
                            stats.emission_rate =
                                (bot.fleet_acct.0.stats.cargo_stats.mining_rate as f32 / 10000.0)
                                    * system_richness
                                    / resource_hardness;

                            // calculate resource amount to extract
                            stats.resource_amount =
                                bot.fleet_acct.0.stats.cargo_stats.cargo_capacity as f32; // minus 'food'

                            // calculate mining duration
                            stats.mining_duration = stats.resource_amount / stats.emission_rate;
                            stats.mining_end_time = time + stats.mining_duration as f64;
                        };

                        stats.is_mining_state = true; // set mining state to true
                        stats.mining_countdown = stats.mining_end_time - time; // mining countdown

                        if is_mouse_button_pressed(MouseButton::Left)
                            || stats.mining_countdown <= 0.0
                        {
                            info!("Prepare to stop mining asteroid");

                            if let Ok(signature) = game_handler.stop_mining_asteroid(bot) {
                                info!("Mining Stop Signature: {:?}", signature);
                                stats.is_mining_state = false; // set mining state to false (reset)

                                bot.fleet_state_dirty = true;
                                bot.set_next_action(Autoplay::ReadyStarbaseDock);
                            }
                        }
                    }
                    FleetState::StarbaseLoadingBay(state) => {
                        ui::print_input("Hangar Withdraw Cargo and Resupply (Mouse Left) | Undock from Starbase (Mouse Right)");

                        if is_mouse_button_pressed(MouseButton::Left)
                            || bot.is_autoplay(Autoplay::ManageHangarCargo)
                        {
                            info!(
                                "Prepare to withdarw cargo, refuel, rearm, and supply cargo hold"
                            );
                            let mut hangar_ok = true;

                            // 1. Withdraw (Hydrogen) from fleet
                            {
                                let pubkey = get_associated_token_address(
                                    &bot.fleet_acct.0.cargo_hold,
                                    &bot.resource,
                                );
                                if let Ok(balance) = rpc_client.get_token_account_balance(&pubkey) {
                                    let mut amount = balance.ui_amount.unwrap_or(0.0) as u64;

                                    if amount >= 2 {
                                        amount -= 1; // leave 1 for associated token account
                                        self.resource_counter += amount; // increment resource counter

                                        if let Err(err) = game_handler.withdraw_from_fleet(
                                            bot,
                                            &state.starbase,
                                            &bot.resource,
                                            amount,
                                        ) {
                                            error!("Error: {:?}", err);
                                            hangar_ok = false;
                                        };
                                    }
                                }
                            }

                            // 2. Fuel tank refuel
                            {
                                let fuel_capcity =
                                    bot.fleet_acct.0.stats.cargo_stats.fuel_capacity as f64;
                                let pubkey = get_associated_token_address(
                                    &bot.fleet_acct.0.fuel_tank,
                                    &game_handler.game_acct.0.mints.fuel,
                                );
                                let balance = rpc_client.get_token_account_balance(&pubkey)?;
                                let amount = balance.ui_amount.unwrap_or(0.0);
                                let tank_usage = amount / fuel_capcity;

                                debug!("Fuel Tank Usage: {}", tank_usage);

                                if tank_usage < 0.5 {
                                    let fuel_tank = &bot.fleet_acct.0.fuel_tank;
                                    let fuel_mint = &game_handler.game_acct.0.mints.fuel;
                                    let fuel_amount = (fuel_capcity - amount) as u64;

                                    debug!("Fuel Amount (Refuel): {}", fuel_amount);

                                    if let Err(err) = game_handler.depost_to_fleet(
                                        bot,
                                        &state.starbase,
                                        fuel_tank,
                                        fuel_mint,
                                        fuel_amount,
                                    ) {
                                        error!("Error: {:?}", err);
                                        hangar_ok = false;
                                    };
                                }
                            }

                            // 3. Ammo bank rearm
                            {
                                let ammo_capcity =
                                    bot.fleet_acct.0.stats.cargo_stats.ammo_capacity as f64;
                                let pubkey = get_associated_token_address(
                                    &bot.fleet_acct.0.ammo_bank,
                                    &game_handler.game_acct.0.mints.ammo,
                                );
                                let balance = rpc_client.get_token_account_balance(&pubkey)?;
                                let amount = balance.ui_amount.unwrap_or(0.0);
                                let bank_usage = amount / ammo_capcity;

                                debug!("Ammo Bank Usage: {}", bank_usage);

                                if bank_usage < 0.5 {
                                    let ammo_bank = &bot.fleet_acct.0.ammo_bank;
                                    let ammo_mint = &game_handler.game_acct.0.mints.ammo;
                                    let ammo_amount = (ammo_capcity - amount) as u64;

                                    debug!("Ammo Amount (Rearm): {}", ammo_amount);

                                    if let Err(err) = game_handler.depost_to_fleet(
                                        bot,
                                        &state.starbase,
                                        ammo_bank,
                                        ammo_mint,
                                        ammo_amount,
                                    ) {
                                        error!("Error: {:?}", err);
                                        hangar_ok = false;
                                    };
                                }
                            }

                            // 4. Cargo hold supply
                            {
                                let cargo_capcity =
                                    bot.fleet_acct.0.stats.cargo_stats.cargo_capacity as f64;
                                let pubkey = get_associated_token_address(
                                    &bot.fleet_acct.0.cargo_hold,
                                    &game_handler.game_acct.0.mints.food,
                                );
                                let balance = rpc_client.get_token_account_balance(&pubkey)?;
                                let amount = balance.ui_amount.unwrap_or(0.0);
                                let hold_usage = amount / cargo_capcity;

                                debug!("Cargo Hold Usage: {}", hold_usage);

                                if hold_usage < 0.05 {
                                    let cargo_hold = &bot.fleet_acct.0.cargo_hold;
                                    let food_mint = &game_handler.game_acct.0.mints.food;
                                    let food_amount = (cargo_capcity * 0.05) as u64;

                                    debug!("Food Amount (Supply): {}", food_amount);

                                    if let Err(err) = game_handler.depost_to_fleet(
                                        bot,
                                        &state.starbase,
                                        cargo_hold,
                                        food_mint,
                                        food_amount,
                                    ) {
                                        error!("Error: {:?}", err);
                                        hangar_ok = false;
                                    };
                                }
                            }

                            if hangar_ok {
                                bot.fleet_state_dirty = true;
                                bot.set_next_action(Autoplay::ReadyStarbaseUndock);
                            }
                        }

                        if is_mouse_button_pressed(MouseButton::Right)
                            || bot.is_autoplay(Autoplay::ReadyStarbaseUndock)
                        {
                            info!("Prepare to undock from starbase");

                            if let Ok(signature) = game_handler.undock_from_starbase(bot) {
                                info!("Undock Signature: {:?}", signature);

                                bot.fleet_state_dirty = true;
                                bot.set_next_action(Autoplay::StartMiningAsteroid);
                            };
                        }
                    }
                    _ => (),
                }
            }

            if is_key_pressed(KeyCode::Escape) {
                info!("Exiting...");
                break;
            }

            next_frame().await
        }

        Ok(())
    }
}

pub async fn run(game_handler: &sage::GameHandler, bots: Vec<bot::Bot>) -> anyhow::Result<()> {
    let mut app = App {
        bots: bots
            .into_iter()
            .map(|bot| (bot, BotStats::default()))
            .collect(),
        resource_counter: 0,
    };
    app.run(game_handler).await
}
