use macroquad::prelude::*;
use spl_associated_token_account::get_associated_token_address;

use staratlas_sage_sdk::{derive, FleetState};

use std::str::FromStr;
use std::time::{Duration, Instant};

use crate::{bot, sage, time, ui};

#[derive(Debug, PartialEq)]
pub enum Autoplay {
    Undefined,
    ManageHangarCargo,
    ReadyStarbaseDock,
    ReadyStarbaseUndock,
    StartMiningAsteroid,
    IsMiningAstroid,
}

struct BotArgs {
    emission_rate: f32,
    resource_amount: f32,
    autoplay_timer: time::Stopwatch,
    autoplay_last_time: Instant,
    mining_timer: time::Timer,
    mining_last_time: Instant,
    next_action: Autoplay,
    fleet_state_dirty: bool,
    errors_counter: u32,
}

impl BotArgs {
    fn new() -> Self {
        BotArgs {
            emission_rate: 0.0,
            resource_amount: 0.0,
            autoplay_timer: time::Stopwatch::default(),
            autoplay_last_time: Instant::now(),
            mining_timer: time::Timer::default(),
            mining_last_time: Instant::now(),
            next_action: Autoplay::Undefined,
            fleet_state_dirty: false,
            errors_counter: 0,
        }
    }
}

impl BotArgs {
    pub fn is_autoplay(&self, next_action: Autoplay) -> bool {
        !self.autoplay_timer.paused() && self.next_action == next_action
    }

    pub fn set_next_action(&mut self, next_action: Autoplay) {
        self.fleet_state_dirty = true;
        self.next_action = next_action;
    }
}

pub struct App {
    bots: Vec<(bot::Bot, BotArgs)>,
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
                    &format!("Elapsed Time: {:.2}", time),
                    &format!("-----------------------------"),
                ],
                0.0,
            );

            for (pos, (bot, args)) in &mut self.bots.iter_mut().enumerate() {
                // calculate autoplay delta time
                let dt = {
                    let now = Instant::now();
                    let delta = now.duration_since(args.autoplay_last_time);
                    args.autoplay_last_time = now;
                    delta
                };
                args.autoplay_timer.tick(dt);

                let y_offset = 60.0 + (pos * 200) as f32;
                ui::print_lines(
                    vec![
                        &format!("Fleet ID: {}", &bot.fleet_id),
                        &format!("Fleet State: {:?}", &bot.fleet_state),
                        &format!(
                            "Mining: Rate({:.3}) Amount({}) Duration({:?})",
                            &args.emission_rate,
                            &args.resource_amount,
                            &args.mining_timer.duration()
                        ),
                        &format!("Mining Elapsed: {:.2}", &args.mining_timer.elapsed_secs()),
                        &format!(
                            "Mining Remaining : {:.3}",
                            &args.mining_timer.remaining_secs()
                        ),
                        &format!("Mining Fraction: {:.3}", &args.mining_timer.fraction()),
                        &format!("Mining Finished: {}", &args.mining_timer.finished()),
                        &format!(
                            "Autoplay ({:?}): {:?}",
                            &args.next_action, &args.autoplay_timer
                        ),
                        &format!("Errors Counter: {}", &args.errors_counter),
                        &format!("-----------------------------"),
                    ],
                    y_offset,
                );

                if args.fleet_state_dirty {
                    // 1. Refresh fleet state (if dirty)
                    let (_, fleet_state) = derive::fleet_account_with_state(
                        &game_handler.sage_program,
                        &bot.fleet_id,
                    )?;

                    bot.fleet_state = fleet_state;
                }

                match bot.fleet_state {
                    FleetState::Idle(idle) => match idle.sector {
                        [-40, 30] | [0, -39] | [40, 30] => {
                            ui::print_input(
                                "Dock to Starbase (Mouse Left) | Mine Asteroid (Mouse Right)",
                            );

                            if is_mouse_button_pressed(MouseButton::Left)
                                || args.is_autoplay(Autoplay::ReadyStarbaseDock)
                            {
                                info!("Prepare to dock to starbase");

                                match game_handler.dock_to_starbase(bot) {
                                    Ok(signature) => {
                                        info!("Dock Signature: {:?}", signature);
                                        args.set_next_action(Autoplay::ManageHangarCargo);
                                    }
                                    Err(err) => {
                                        error!("Error: {:?}", err);
                                        args.errors_counter += 1;
                                    }
                                }
                            }

                            if is_mouse_button_pressed(MouseButton::Right)
                                || args.is_autoplay(Autoplay::StartMiningAsteroid)
                            {
                                info!("Prepare to mine asteroid");

                                match game_handler.start_mining_asteroid(bot) {
                                    Ok(signature) => {
                                        info!("Mining Start Signature: {:?}", signature);
                                        args.set_next_action(Autoplay::IsMiningAstroid);
                                    }
                                    Err(err) => {
                                        error!("Error: {:?}", err);
                                        args.errors_counter += 1;
                                    }
                                }
                            }
                        }
                        _ => unimplemented!(),
                    },
                    FleetState::MineAsteroid(mine_asteroid) => {
                        ui::print_input("Stop Mine Asteroid (Mouse Left)");

                        // a zero mining timer duration means this is the 'first' time mining
                        if args.mining_timer.duration() == Duration::ZERO {
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
                            args.emission_rate =
                                (bot.fleet_acct.0.stats.cargo_stats.mining_rate as f32 / 10000.0)
                                    * system_richness
                                    / resource_hardness;

                            // calculate resource amount to extract
                            let keyed_accounts = rpc_client.get_token_accounts_by_owner(
                                &bot.fleet_acct.0.cargo_hold,
                                anchor_client::solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::id()),
                            )?;

                            // calculate 'estimated' space left in cargo hold
                            let held_amount =
                                keyed_accounts.iter().fold(0.0, |amount, keyed_acct| {
                                    let pubkey =
                                        anchor_client::anchor_lang::prelude::Pubkey::from_str(
                                            &keyed_acct.pubkey,
                                        )
                                        .unwrap();
                                    let balance =
                                        rpc_client.get_token_account_balance(&pubkey).unwrap();

                                    let ui_amount = balance.ui_amount.unwrap_or(0.0);
                                    amount + ui_amount
                                }) as u32;

                            let cargo_capacity = bot.fleet_acct.0.stats.cargo_stats.cargo_capacity;
                            let mining_time_elapsed = time::get_time() as i64 - mine_asteroid.start;
                            let amount_mined = mining_time_elapsed as f32 * args.emission_rate;
                            let mut est_held_cargo = held_amount + amount_mined as u32;
                            est_held_cargo = est_held_cargo.min(cargo_capacity);

                            args.resource_amount = (cargo_capacity - est_held_cargo) as f32;

                            // calculate mining duration and set mining timer
                            let mining_duration =
                                Duration::from_secs_f32(args.resource_amount / args.emission_rate);
                            args.mining_timer.set_duration(mining_duration);
                        };

                        // calculate mining delta time
                        let dt = {
                            let now = Instant::now();
                            let delta = now.duration_since(args.mining_last_time);
                            args.mining_last_time = now;
                            delta
                        };

                        // update mining timer
                        args.mining_timer.tick(dt);

                        if is_mouse_button_pressed(MouseButton::Left)
                            || args.mining_timer.finished()
                        {
                            info!("Prepare to stop mining asteroid");

                            match game_handler.stop_mining_asteroid(bot) {
                                Ok(signature) => {
                                    info!("Mining Stop Signature: {:?}", signature);
                                    // set mining timer duration to zero and reset
                                    args.mining_timer.set_duration(Duration::ZERO);
                                    args.mining_timer.reset();
                                    args.set_next_action(Autoplay::ReadyStarbaseDock);
                                }
                                Err(err) => {
                                    error!("Error: {:?}", err);
                                    args.errors_counter += 1;
                                }
                            }
                        }
                    }
                    FleetState::StarbaseLoadingBay(state) => {
                        ui::print_input("Hangar Withdraw Cargo and Resupply (Mouse Left) | Undock from Starbase (Mouse Right)");

                        if is_mouse_button_pressed(MouseButton::Left)
                            || args.is_autoplay(Autoplay::ManageHangarCargo)
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
                                            args.errors_counter += 1;
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
                                        args.errors_counter += 1;
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
                                        args.errors_counter += 1;
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
                                        args.errors_counter += 1;
                                        hangar_ok = false;
                                    };
                                }
                            }

                            if hangar_ok {
                                args.set_next_action(Autoplay::ReadyStarbaseUndock);
                            }
                        }

                        if is_mouse_button_pressed(MouseButton::Right)
                            || args.is_autoplay(Autoplay::ReadyStarbaseUndock)
                        {
                            info!("Prepare to undock from starbase");

                            match game_handler.undock_from_starbase(bot) {
                                Ok(signature) => {
                                    info!("Undock Signature: {:?}", signature);
                                    args.set_next_action(Autoplay::StartMiningAsteroid);
                                }
                                Err(err) => {
                                    error!("Error: {:?}", err);
                                    args.errors_counter += 1;
                                }
                            }
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

pub async fn run(
    game_handler: &sage::GameHandler,
    bots: Vec<bot::Bot>,
    autoplay: bool,
) -> anyhow::Result<()> {
    let mut bots: Vec<_> = bots.into_iter().map(|bot| (bot, BotArgs::new())).collect();

    if !autoplay {
        bots.iter_mut().for_each(|(_, args)| {
            args.autoplay_timer.pause();
        });
    }

    let mut app = App {
        bots,
        resource_counter: 0,
    };

    app.run(game_handler).await
}
