use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster, Program,
};
use clap::Parser;
use macroquad::prelude::*;
use solana_program::pubkey;
use spl_associated_token_account::get_associated_token_address;

use staratlas_sage_sdk::{
    derive, ixs,
    programs::{staratlas_cargo::ID as CARGO_ID, staratlas_sage::ID as SAGE_ID},
    FleetState,
};

use std::ops::Deref;
use std::rc::Rc;

const HYDRO_MINT: Pubkey = pubkey!("HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp");

#[derive(Debug, PartialEq)]
enum Autoplay {
    Unknown,
    HangarManageCargo,
    StarbaseDock,
    StarbaseUndock,
    StartMiningAsteroid,
}

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    provider_config: ProviderConfig,
    #[clap(flatten)]
    sage_config: SageConfig,
}

#[derive(Default, Parser)]
struct ProviderConfig {
    /// RPC URL for the Solana cluster.
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    pub cluster: Option<Cluster>,
    /// Wallet keypair to use.
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    wallet: Option<String>,
}

#[derive(Default, Parser)]
struct SageConfig {
    /// Sage Game's Pubkey
    #[clap(long = "sage.game_id", env = "SAGE_GAME_ID")]
    game_id: Option<Pubkey>,
    /// Sage Player Profile's Pubkey
    #[clap(long = "sage.profile_id", env = "SAGE_PROFILE_ID")]
    profile_id: Option<Pubkey>,
    /// Sage Fleet's Pubkey
    #[clap(long = "sage.fleet_id", env = "SAGE_FLEET_ID")]
    fleet_id: Option<Pubkey>,
}

fn default_keypair() -> Keypair {
    read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Requires a keypair file")
}

fn parse_sage_config(sage_config: &SageConfig) -> (Pubkey, Pubkey) {
    let game_id = sage_config
        .game_id
        .expect("Requires --sage.game_state_id <GAME_STATE_ID>");

    let profile_id = sage_config
        .profile_id
        .expect("Requires --sage.profile_id <PROFILE_ID>");

    (game_id, profile_id)
}

fn send_and_sign<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    ixs: Vec<Instruction>,
) -> anyhow::Result<()> {
    let mut builder = program.request();

    for ix in ixs {
        builder = builder.instruction(ix);
    }

    let signature = builder.send()?;
    info!("Signature: {}", signature);

    Ok(())
}

fn print_input(text: &str) {
    let center = get_text_center(text, Option::None, 30, 1.0, 0.0);
    draw_text_ex(
        text,
        screen_width() / 2.0 - center.x,
        screen_height() / 2.0 - center.y,
        TextParams {
            font_size: 30,
            ..Default::default()
        },
    );
}

#[macroquad::main("HydroBot")]
async fn main() -> anyhow::Result<()> {
    info!("Mine hydrogen with HydroBot!");

    let cli = Cli::parse();

    let payer = match cli.provider_config.wallet {
        Some(wallet) => read_keypair_file(wallet).expect("Requires a keypair file"),
        None => default_keypair(),
    };

    let url = match cli.provider_config.cluster {
        Some(cluster) => cluster,
        None => Cluster::Devnet,
    };

    let client = Client::new_with_options(
        url,
        Rc::new(Keypair::from_bytes(&payer.to_bytes())?),
        CommitmentConfig::confirmed(),
    );

    let sage_program = client.program(SAGE_ID)?;
    let cargo_program = client.program(CARGO_ID)?;
    let rpc_client = sage_program.rpc();

    let (game_id, player_profile) = parse_sage_config(&cli.sage_config);

    let fleet_id = &cli
        .sage_config
        .fleet_id
        .expect("Requires --sage.fleet_id <FLEET_ID>");

    info!("Game ID: {}", &game_id);
    info!("PLayer Profile: {}", &player_profile);
    info!("Fleet ID: {}", fleet_id);

    let game = derive::game_account(&sage_program, &game_id)?;
    let (fleet, mut fleet_state) = derive::fleet_account_with_state(&sage_program, &fleet_id)?;
    debug!("{:#?}", (&fleet, &fleet_state));

    let fleet_id_text = format!("Fleet ID: {}", fleet_id);
    let fleet_cargo_stats_text = format!("Fleet Stats: {:?}", &fleet.0.stats.cargo_stats);
    let mut resource_counter: u64 = 0;

    let mut is_mining_state = false;
    let mut emission_rate: f32 = 0.0;
    let mut resource_amount: f32 = 0.0;
    let mut mining_duration: f32 = 0.0;
    let mut mining_end_time: f64 = 0.0;

    let autoplay = false;
    let mut next_action = Autoplay::Unknown;

    loop {
        clear_background(BLUE);
        let time = get_time();

        draw_text_ex(&fleet_id_text, 20.0, 20.0, TextParams::default());
        draw_text_ex(&fleet_cargo_stats_text, 20.0, 40.0, TextParams::default());
        draw_text_ex(
            &format!("Fleet State: {:?}", &fleet_state),
            20.0,
            60.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Resource: {}", &HYDRO_MINT),
            20.0,
            80.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Counter: {}", &resource_counter),
            20.0,
            100.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Is Mining?: {:?}", &is_mining_state),
            20.0,
            120.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Emission Rate: {}", &emission_rate),
            20.0,
            140.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Resource Amount: {}", &resource_amount),
            20.0,
            160.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Mining Duration: {}", &mining_duration),
            20.0,
            180.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Mining End Time: {}", &mining_end_time),
            20.0,
            200.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Elapsed Time: {:2}", time),
            20.0,
            220.0,
            TextParams::default(),
        );
        draw_text_ex(
            &format!("Autoplay ({:?}): {:?}", autoplay, next_action),
            20.0,
            240.0,
            TextParams::default(),
        );

        match &fleet_state {
            FleetState::Idle(idle) => match idle.sector {
                [-40, 30] | [0, -39] | [40, 30] => {
                    print_input("Dock to Starbase (Mouse Left) | Mine Asteroid (Mouse Right)");

                    if is_mouse_button_pressed(MouseButton::Left)
                        || (autoplay && next_action == Autoplay::StarbaseDock)
                    {
                        info!("Prepare to dock to starbase");

                        // 1. Dock to starbase
                        let ixs = ixs::starbase::dock_to_starbase(
                            &sage_program,
                            (fleet_id, (&fleet, &fleet_state)),
                            (&game_id, &game),
                        )?;

                        send_and_sign(&sage_program, ixs)?;

                        // 2. Refresh fleet state
                        let (_, new_fleet_state) =
                            derive::fleet_account_with_state(&sage_program, &fleet_id)?;
                        fleet_state = new_fleet_state;
                        next_action = Autoplay::HangarManageCargo;
                    }

                    if is_mouse_button_pressed(MouseButton::Right)
                        || (autoplay && next_action == Autoplay::StartMiningAsteroid)
                    {
                        info!("Prepare to mine asteroid");

                        // 1. Start mining
                        let ixs = ixs::mine::start_mining_asteroid(
                            &sage_program,
                            (fleet_id, (&fleet, &fleet_state)),
                            (&game_id, &game),
                        )?;

                        send_and_sign(&sage_program, ixs)?;

                        // 2. Refresh fleet state
                        let (_, new_fleet_state) =
                            derive::fleet_account_with_state(&sage_program, &fleet_id)?;
                        fleet_state = new_fleet_state;
                    }
                }
                _ => (),
            },
            FleetState::MineAsteroid(_mine_asteroid) => {
                print_input("Stop Mine Asteroid (Mouse Left)");

                if !is_mining_state {
                    // // mine asteroid's resource (account) and mine item (account)
                    // let resource =
                    //     derive_account::<_, state::Resource>(&sage_program, &mine_asteroid.resource)?;
                    // let mine_item =
                    //     derive_account::<_, state::MineItem>(&sage_program, &resource.mine_item)?;

                    // dbg!(mine_item.resource_hardness);
                    // dbg!(resource.system_richness);
                    let resource_hardness: f32 = 100.0 / 100.0;
                    let system_richness: f32 = 100.0 / 100.0;

                    // calculate asteroid mining emission rate
                    emission_rate = (fleet.0.stats.cargo_stats.mining_rate as f32 / 10000.0)
                        * system_richness
                        / resource_hardness;

                    // calculate resource amount to extract
                    resource_amount = fleet.0.stats.cargo_stats.cargo_capacity as f32; // minus 'food'

                    // calculate mining duration
                    mining_duration = resource_amount / emission_rate;
                    mining_end_time = time + mining_duration as f64;
                };

                is_mining_state = true; // set mining state to true
                let mining_cooldown = mining_end_time - time; // mining cooldown

                if is_mouse_button_pressed(MouseButton::Left) || mining_cooldown <= 0.0 {
                    info!("Prepare to stop mining asteroid");

                    // 1. Stop mining
                    let ixs = ixs::mine::stop_mining_asteroid(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
                    )?;

                    send_and_sign(&sage_program, ixs)?;

                    // 2. Refresh fleet state
                    let (_, new_fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;
                    fleet_state = new_fleet_state;

                    is_mining_state = false; // set mining state to false (reset)
                    next_action = Autoplay::StarbaseDock;
                }
            }
            FleetState::StarbaseLoadingBay(state) => {
                print_input("Hangar Withdraw Cargo and Resupply (Mouse Left) | Undock from Starbase (Mouse Right)");

                if is_mouse_button_pressed(MouseButton::Left)
                    || (autoplay && next_action == Autoplay::HangarManageCargo)
                {
                    info!("Prepare to withdarw cargo, refuel, rearm, and supply cargo hold");

                    // 1. Withdraw (Hydrogen) from fleet
                    let pubkey = get_associated_token_address(&fleet.0.cargo_hold, &HYDRO_MINT);
                    if let Ok(balance) = rpc_client.get_token_account_balance(&pubkey) {
                        let mut amount = balance.ui_amount.unwrap_or(0.0) as u64;

                        if amount >= 2 {
                            amount -= 1; // leave 1 for associated token account
                            resource_counter += amount; // increment resource counter

                            let ixs = ixs::cargo::withdraw_from_fleet(
                                &sage_program,
                                &cargo_program,
                                (fleet_id, &fleet),
                                (&game_id, &game),
                                &state.starbase,
                                &HYDRO_MINT,
                                amount,
                            )?;

                            send_and_sign(&sage_program, ixs)?;
                        }
                    }

                    // 2. Fuel tank refuel
                    let fuel_capcity = fleet.0.stats.cargo_stats.fuel_capacity as f64;
                    let pubkey =
                        get_associated_token_address(&fleet.0.fuel_tank, &game.0.mints.fuel);
                    let balance = rpc_client.get_token_account_balance(&pubkey)?;
                    let amount = balance.ui_amount.unwrap_or(0.0);
                    let tank_usage = amount / fuel_capcity;

                    debug!("Fuel Tank Usage: {}", tank_usage);

                    if tank_usage < 0.5 {
                        let fuel_tank = &fleet.0.fuel_tank;
                        let mint = &game.0.mints.fuel;
                        let refuel = (fuel_capcity - amount) as u64;

                        debug!("Refuel: {}", refuel);

                        let ixs = ixs::cargo::depost_to_fleet(
                            &sage_program,
                            &cargo_program,
                            (fleet_id, &fleet),
                            (&game_id, &game),
                            &state.starbase,
                            fuel_tank,
                            mint,
                            refuel,
                        )?;

                        send_and_sign(&sage_program, ixs)?;
                    }

                    // 3. Ammo bank rearm
                    let ammo_capcity = fleet.0.stats.cargo_stats.ammo_capacity as f64;
                    let pubkey =
                        get_associated_token_address(&fleet.0.ammo_bank, &game.0.mints.ammo);
                    let balance = rpc_client.get_token_account_balance(&pubkey)?;
                    let amount = balance.ui_amount.unwrap_or(0.0);
                    let bank_usage = amount / ammo_capcity;

                    debug!("Ammo Bank Usage: {}", bank_usage);

                    if bank_usage < 0.5 {
                        let ammo_bank = &fleet.0.ammo_bank;
                        let mint = &game.0.mints.ammo;
                        let rearm = (ammo_capcity - amount) as u64;

                        debug!("Rearm: {}", rearm);

                        let ixs = ixs::cargo::depost_to_fleet(
                            &sage_program,
                            &cargo_program,
                            (fleet_id, &fleet),
                            (&game_id, &game),
                            &state.starbase,
                            ammo_bank,
                            mint,
                            rearm,
                        )?;

                        send_and_sign(&sage_program, ixs)?;
                    }

                    // 3. Cargo hold supply
                    let cargo_capcity = fleet.0.stats.cargo_stats.cargo_capacity as f64;
                    let pubkey =
                        get_associated_token_address(&fleet.0.cargo_hold, &game.0.mints.food);
                    let balance = rpc_client.get_token_account_balance(&pubkey)?;
                    let amount = balance.ui_amount.unwrap_or(0.0);
                    let hold_usage = amount / cargo_capcity;

                    debug!("Cargo Hold Usage: {}", hold_usage);

                    if hold_usage < 0.05 {
                        let cargo_hold = &fleet.0.cargo_hold;
                        let mint = &game.0.mints.food;
                        let supply = (cargo_capcity * 0.05) as u64;

                        debug!("Supply: {}", supply);

                        let ixs = ixs::cargo::depost_to_fleet(
                            &sage_program,
                            &cargo_program,
                            (fleet_id, &fleet),
                            (&game_id, &game),
                            &state.starbase,
                            cargo_hold,
                            mint,
                            supply,
                        )?;

                        send_and_sign(&sage_program, ixs)?;
                    }

                    next_action = Autoplay::StarbaseUndock;
                }

                if is_mouse_button_pressed(MouseButton::Right)
                    || (autoplay && next_action == Autoplay::StarbaseUndock)
                {
                    info!("Prepare to undock from starbase");

                    // 1. Undock from starbase
                    let ixs = ixs::starbase::undock_from_starbase(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
                    )?;

                    send_and_sign(&sage_program, ixs)?;

                    // 2. Refresh fleet state
                    let (_, new_fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;
                    fleet_state = new_fleet_state;

                    next_action = Autoplay::StartMiningAsteroid;
                }
            }
            _ => (),
        }

        if is_key_pressed(KeyCode::Escape) {
            info!("Exiting...");
            break;
        }

        next_frame().await
    }

    Ok(())
}
