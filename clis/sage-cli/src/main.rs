use anchor_client::{
    solana_client::rpc_response::{Response, RpcSimulateTransactionResult},
    solana_sdk::{
        commitment_config::CommitmentConfig,
        compute_budget::ComputeBudgetInstruction,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    },
    Client, Cluster, Program, RequestBuilder,
};
use clap::{Parser, Subcommand};

use staratlas_player_profile_sdk::{
    derive::profile_accounts as derive_profile_accounts,
    programs::staratlas_player_profile::ID as PLAYER_PROFILE_ID,
};
use staratlas_sage_sdk::{
    accounts::FleetState,
    derive, ixs,
    programs::{
        staratlas_cargo::ID as CARGO_ID, staratlas_points::ID as POINTS_ID,
        staratlas_sage::ID as SAGE_ID,
    },
};

// use std::io::{self, Write};
use std::ops::Deref;
use std::rc::Rc;

/// Star Atlas: Sage CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    provider_config: ProviderConfig,
    #[clap(flatten)]
    sage_config: SageConfig,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Default, Parser)]
struct ProviderConfig {
    /// RPC URL for the Solana cluster.
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    pub cluster: Option<Cluster>,
    /// Wallet keypair to use.
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    wallet: Option<String>,
}

#[derive(Debug, Default, Parser)]
struct SageConfig {
    /// Sage Game's Pubkey
    #[clap(long = "sage.game_id", env = "SAGE_GAME_ID")]
    game_id: Option<Pubkey>,
    /// Sage Player Profile's Pubkey
    #[clap(long = "sage.profile_id", env = "SAGE_PROFILE_ID")]
    profile_id: Option<Pubkey>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(subcommand)]
    Actions(Actions),
    #[command(subcommand)]
    Find(Find),
    #[command(subcommand)]
    Show(Show),
}

#[derive(Debug, Subcommand, PartialEq)]
enum Actions {
    CargoDeposit {
        fleet_id: Pubkey,
        mint: Pubkey,
        amount: u64,
    },
    CargoWithdraw {
        fleet_id: Pubkey,
        mint: Pubkey,
        amount: u64,
    },
    StarbaseDock {
        fleet_id: Pubkey,
    },
    StarbaseUndock {
        fleet_id: Pubkey,
    },
    StartMining {
        fleet_id: Pubkey,
        mine_item: Option<Pubkey>,
    },
    StopMining {
        fleet_id: Pubkey,
    },
    Warp {
        fleet_id: Pubkey,
        x_coord: i64,
        y_coord: i64,
    },
    WarpExit {
        fleet_id: Pubkey,
    },
}

#[derive(Debug, Subcommand)]
enum Find {
    Games,
    Fleet {
        /// Fleet's Label
        fleet_name: String,
    },
    PlayerProfile,
    Planets {
        x: i64,
        y: i64,
    },
    PointsModifiers,
}

#[derive(Debug, Subcommand)]
enum Show {
    AllFleets,
    Fleet {
        /// Fleet's Pubkey
        fleet_id: Pubkey,
        /// Show Fleet's State (default: false)
        #[arg(long, default_value_t = false)]
        with_state: bool,
    },
    Game {
        /// Game's Pubkey
        game_id: Pubkey,
    },
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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // dbg!(&cli);

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
    let player_profile_program = client.program(PLAYER_PROFILE_ID)?;
    let cargo_program = client.program(CARGO_ID)?;
    let points_program = client.program(POINTS_ID)?;

    let (game_id, player_profile) = parse_sage_config(&cli.sage_config);

    match &cli.command {
        Commands::Actions(action) => {
            let game = derive::game_account(&sage_program, &game_id)?;

            let ixs = match action {
                Actions::CargoDeposit {
                    fleet_id,
                    mint,
                    amount,
                } => {
                    // ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK
                    // foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG
                    // fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let cargo_pod_to = if mint == &game.mints.fuel {
                        &fleet.fuel_tank
                    } else if mint == &game.mints.ammo {
                        &fleet.ammo_bank
                    } else {
                        &fleet.cargo_hold
                    };

                    match fleet_state {
                        FleetState::StarbaseLoadingBay(state) => {
                            let ixs = ixs::cargo::deposit_to_fleet(
                                &sage_program,
                                &cargo_program,
                                (fleet_id, &fleet),
                                (&game_id, &game),
                                &state.starbase,
                                cargo_pod_to,
                                mint,
                                *amount,
                            )?;

                            Some(ixs)
                        }
                        _ => {
                            println!("Fleet is not docked at a starbase");
                            None
                        }
                    }
                }
                Actions::CargoWithdraw {
                    fleet_id,
                    mint,
                    amount,
                } => {
                    // HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    match fleet_state {
                        FleetState::StarbaseLoadingBay(state) => {
                            let ixs = ixs::cargo::withdraw_from_fleet(
                                &sage_program,
                                &cargo_program,
                                (fleet_id, &fleet),
                                (&game_id, &game),
                                &state.starbase,
                                mint,
                                *amount,
                            )?;

                            Some(ixs)
                        }
                        _ => {
                            println!("Fleet is not docked at a starbase");
                            None
                        }
                    }
                }
                Actions::StarbaseDock { fleet_id } => {
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::starbase::dock_to_starbase(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
                    )?;

                    Some(ixs)
                }
                Actions::StarbaseUndock { fleet_id } => {
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::starbase::undock_from_starbase(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
                    )?;

                    Some(ixs)
                }
                Actions::StartMining {
                    fleet_id,
                    mine_item,
                } => {
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::mine::start_mining_asteroid(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
                        *mine_item,
                    )?;

                    Some(ixs)
                }
                Actions::StopMining { fleet_id } => {
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::mine::stop_mining_asteroid(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
                    )?;

                    Some(ixs)
                }
                Actions::Warp {
                    fleet_id,
                    x_coord,
                    y_coord,
                } => {
                    let (fleet, _fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::warp::warp_to_coordinate(
                        &sage_program,
                        &cargo_program,
                        (fleet_id, &fleet),
                        (&game_id, &game),
                        [*x_coord, *y_coord],
                    )?;

                    Some(ixs)
                }
                Actions::WarpExit { fleet_id } => {
                    let (fleet, _fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::warp::ready_to_exit_warp(
                        &sage_program,
                        (fleet_id, &fleet),
                        (&game_id, &game),
                    )?;

                    Some(ixs)
                }
            };

            // print!("Confirm sign and send? Y/N: ");
            // io::stdout().flush()?;

            // let mut input = String::new();
            // io::stdin().read_line(&mut input)?;

            // if input.trim().eq_ignore_ascii_case("Y") {
            //     let signature = builder.send()?;
            //     println!("{}", signature);
            // } else {
            //     let tx = builder.signed_transaction()?;
            //     dbg!(tx);
            // }

            pub fn simulate_transaction<C: Deref<Target = impl Signer> + Clone>(
                builder: &RequestBuilder<C>,
                program: &Program<C>,
                signers: &Vec<&dyn Signer>,
            ) -> Result<Response<RpcSimulateTransactionResult>, Box<dyn std::error::Error>>
            {
                let instructions = builder.instructions()?;
                let rpc_client = program.rpc();
                let recent_blockhash = rpc_client.get_latest_blockhash()?;
                let tx = Transaction::new_signed_with_payer(
                    &instructions,
                    Some(&program.payer()),
                    signers,
                    recent_blockhash,
                );
                let simulation = rpc_client.simulate_transaction(&tx)?;
                Ok(simulation)
            }

            // `ixs` either [] (0 txs), [ix] (1 txs) or [ix, ix] (2 txs)
            if let Some(ixs) = ixs {
                for ix in ixs {
                    // let mut simulated = sage_program.request();

                    // simulated = ix
                    //     .clone()
                    //     .into_iter()
                    //     .fold(simulated, |simulated, i| simulated.instruction(i));

                    // let simulation =
                    //     simulate_transaction(&simulated, &sage_program, &vec![&payer]).unwrap();
                    // dbg!(&simulation);

                    // let units_consumed = simulation.value.units_consumed;
                    // dbg!(&units_consumed);

                    let mut builder = sage_program.request();

                    // if let Some(units) = units_consumed {
                    //     let units = (units + 100) as u32; // some margin of error...
                    //     let i = ComputeBudgetInstruction::set_compute_unit_limit(units);
                    //     builder = builder.instruction(i);
                    // }

                    let i = ComputeBudgetInstruction::set_compute_unit_price(5000);
                    builder = builder.instruction(i);

                    builder = ix
                        .into_iter()
                        .fold(builder, |builder, i| builder.instruction(i));

                    // dbg!(&builder.instructions());

                    dbg!("Sending transaction");
                    let signature = builder.send()?;
                    println!("{}", signature);
                }
            }
        }
        Commands::Find(find) => match find {
            Find::Games => {
                let game_accounts = derive::game_accounts(&sage_program)?;

                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Game ID", "Version", "Mints"]);

                for (pubkey, game) in game_accounts {
                    dbg!(&game.points);
                    table.add_row(vec![
                        pubkey.to_string(),
                        game.version.to_string(),
                        format!("{:#?}", game.mints),
                    ]);
                }

                println!("{table}");
            }
            Find::Fleet { fleet_name } => {
                let (fleet_pubkey, _) =
                    derive::fleet_address(&game_id, &player_profile, fleet_name.as_str());

                let fleet = derive::fleet_account(&sage_program, &fleet_pubkey)?;

                println!("{:#?}", vec![(fleet_pubkey, fleet)]);
            }
            Find::PlayerProfile => {
                let profile_accounts =
                    derive_profile_accounts(&player_profile_program, &payer.pubkey())?;

                let mut table = comfy_table::Table::new();
                table.set_header(vec![
                    "Profile ID",
                    "Version",
                    "Auth Key Count",
                    "Key Threshold",
                ]);

                for (pubkey, profile) in profile_accounts {
                    table.add_row(vec![
                        pubkey.to_string(),
                        profile.0.version.to_string(),
                        profile.0.auth_key_count.to_string(),
                        profile.0.key_threshold.to_string(),
                    ]);
                }

                println!("{table}");
            }
            Find::Planets { x, y } => {
                let planets = derive::planet_accounts(&sage_program, &game_id, [*x, *y])?;
                let (pubkey, planet) = planets
                    .into_iter()
                    .find(|(_, planet)| planet.num_resources >= 1)
                    .expect("planet with resources");

                dbg!(&planet);
                let resources = derive::resource_accounts(&sage_program, &game_id, &pubkey)?;

                for (pubkey, resource) in resources {
                    dbg!(pubkey);
                    dbg!(&resource);

                    let mine_item = derive::mine_item_account(&sage_program, &resource.mine_item)?;
                    let name = mine_item.name();

                    dbg!(name);
                    dbg!(mine_item);
                }
            }
            Find::PointsModifiers => {
                let accounts = points_program.accounts::<staratlas_sage_sdk::programs::staratlas_points::state::PointsModifier>(vec![])?;

                for (pubkey, account) in accounts {
                    println!("{:#?}", (pubkey, account.version, account.point_category));
                }
            }
        },
        Commands::Show(show) => {
            match show {
                Show::AllFleets => {
                    let fleet_accounts =
                        derive::fleet_accounts(&sage_program, &game_id, &player_profile)?;

                    let mut table = comfy_table::Table::new();
                    table.set_header(vec![
                        "Fleet ID",
                        "Fleet Label",
                        "Ship Counts",
                        "Movement Stats",
                        "Cargo Stats",
                        "Misc Stats",
                    ]);

                    for (pubkey, fleet) in fleet_accounts {
                        table.add_row(vec![
                            pubkey.to_string(),
                            fleet.fleet_label().to_string(),
                            format!("{:#?}", fleet.ship_counts),
                            format!("{:#?}", fleet.stats.movement_stats),
                            format!("{:#?}", fleet.stats.cargo_stats),
                            format!("{:#?}", fleet.stats.misc_stats),
                        ]);
                    }

                    println!("{table}");
                }
                Show::Fleet {
                    fleet_id,
                    with_state,
                } => {
                    if *with_state {
                        let (fleet, fleet_state) =
                            derive::fleet_account_with_state(&sage_program, &fleet_id)?;
                        println!("{:#?}", (fleet, fleet_state));
                    } else {
                        let fleet = derive::fleet_account(&sage_program, &fleet_id)?;
                        println!("{:#?}", fleet);

                        // let rpc_client = sage_program.rpc();
                        // let keyed_accounts = rpc_client.get_token_accounts_by_owner(
                        //     &fleet.0.cargo_hold,
                        //     anchor_client::solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::id()),
                        // )?;
                        // dbg!(keyed_accounts);
                    }
                }
                Show::Game { game_id } => {
                    let game = derive::game_account(&sage_program, &game_id)?;
                    println!("{:#?}", game);
                }
            }
        }
    }

    Ok(())
}
