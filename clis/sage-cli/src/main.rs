use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster,
};
use anchor_lang::Id;
use clap::{Parser, Subcommand};

use staratlas_player_profile_sdk::{program::PlayerProfile, utils::derive_profile_accounts};
use staratlas_sage_sdk::{
    derive, ixs,
    programs::{staratlas_cargo::program::Cargo, staratlas_sage::program::Sage},
    FleetState,
};

use std::io::{self, Write};
use std::rc::Rc;

/// Star Atlas: Sage CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    provider_config: ProviderConfig,
    #[clap(flatten)]
    sage_config: SageConfig,
    #[command(subcommand)]
    command: Commands,
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
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Actions(Actions),
    #[command(subcommand)]
    Find(Find),
    #[command(subcommand)]
    Show(Show),
}

#[derive(Subcommand)]
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

#[derive(Subcommand)]
enum Find {
    Games,
    Fleet {
        /// Fleet's Label
        fleet_name: String,
    },
    PlayerProfile,
}

#[derive(Subcommand)]
enum Show {
    AllFleets,
    Fleet {
        /// Fleet's Pubkey
        fleet_id: Pubkey,
        /// Show Fleet's State (default: false)
        #[arg(long, default_value_t = false)]
        with_state: bool,
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

    let sage_program = client.program(Sage::id())?;
    let player_profile_program = client.program(PlayerProfile::id())?;

    match &cli.command {
        Commands::Actions(action) => {
            let (game_id, _) = parse_sage_config(&cli.sage_config);

            let cargo_program = client.program(Cargo::id())?;

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

                    let cargo_pod_to = if mint == &game.0.mints.fuel {
                        &fleet.0.fuel_tank
                    } else if mint == &game.0.mints.ammo {
                        &fleet.0.ammo_bank
                    } else {
                        &fleet.0.cargo_hold
                    };

                    match fleet_state {
                        FleetState::StarbaseLoadingBay(state) => {
                            let ixs = ixs::cargo::depost_to_fleet(
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
                Actions::StartMining { fleet_id } => {
                    let (fleet, fleet_state) =
                        derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    let ixs = ixs::mine::start_mining_asteroid(
                        &sage_program,
                        (fleet_id, (&fleet, &fleet_state)),
                        (&game_id, &game),
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

                    let ixs = ixs::warp::ready_to_exit_warp(&sage_program, (fleet_id, &fleet))?;

                    Some(ixs)
                }
            };

            if let Some(ixs) = ixs {
                let mut builder = sage_program.request();
                for ix in ixs {
                    builder = builder.instruction(ix);
                }

                print!("Confirm sign and send? Y/N: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().eq_ignore_ascii_case("Y") {
                    let signature = builder.send()?;
                    println!("{}", signature);
                } else {
                    let ixs = builder.instructions()?;
                    dbg!(ixs);
                }
            }
        }
        Commands::Find(find) => match find {
            Find::Games => {
                let game_accounts = derive::game_accounts(&sage_program)?;

                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Game ID", "Version", "Mints"]);

                for (pubkey, game) in game_accounts {
                    table.add_row(vec![
                        pubkey.to_string(),
                        game.0.version.to_string(),
                        format!("{:#?}", game.0.mints),
                    ]);
                }

                println!("{table}");
            }
            Find::Fleet { fleet_name } => {
                let (game_id, player_profile) = parse_sage_config(&cli.sage_config);

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
        },
        Commands::Show(show) => {
            let (game_id, player_profile) = parse_sage_config(&cli.sage_config);

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
                            format!("{:#?}", fleet.0.ship_counts),
                            format!("{:#?}", fleet.0.stats.movement_stats),
                            format!("{:#?}", fleet.0.stats.cargo_stats),
                            format!("{:#?}", fleet.0.stats.misc_stats),
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
                        // println!("{:#?}", fleet);

                        use std::str::FromStr;
                        let rpc_client = sage_program.rpc();
                        let keyed_accounts = rpc_client.get_token_accounts_by_owner(
                            &fleet.0.cargo_hold,
                            anchor_client::solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token::id()),
                        )?;

                        let total_amount = keyed_accounts.iter().fold(0.0, |amount, keyed_acct| {
                            let pubkey = Pubkey::from_str(&keyed_acct.pubkey).unwrap();
                            let balance = rpc_client.get_token_account_balance(&pubkey).unwrap();

                            dbg!(&keyed_acct.account.data);
                            dbg!(&balance);

                            let ui_amount = balance.ui_amount.unwrap_or(0.0);
                            amount + ui_amount
                        });

                        dbg!(&total_amount);
                    }
                }
            }
        }
    }

    Ok(())
}
