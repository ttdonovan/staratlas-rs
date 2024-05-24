use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
    },
    Client, Cluster,
};
use clap::Parser;

use staratlas_sage_based_sdk::{
    addr, filter, ixs,
    program::{staratlas_sage::state, SAGE_ID},
    FleetState, FleetWithState, SageBasedGameHandler,
};

// use std::io::{self, Write};
use std::rc::Rc;

use sa_sage_cli::{Actions, Cli, Commands, Find, SageConfig, Show};

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    // let player_profile_program = client.program(PLAYER_PROFILE_ID)?;
    // let cargo_program = client.program(CARGO_ID)?;
    // let points_program = client.program(POINTS_ID)?;

    let (game_id, player_profile_id) = parse_sage_config(&cli.sage_config);

    match &cli.command {
        Commands::Actions(action) => {
            let (game_id, game) = SageBasedGameHandler::get_game(&sage_program, &game_id).await?;

            let ixs = match action {
                Actions::CargoDeposit {
                    fleet_id,
                    mint,
                    amount,
                } => {
                    todo!("Actions::CargoDeposit");
                    // ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK
                    // foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG
                    // fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim

                    // let (fleet, fleet_state) =
                    //     derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    // let cargo_pod_to = if mint == &game.mints.fuel {
                    //     &fleet.fuel_tank
                    // } else if mint == &game.mints.ammo {
                    //     &fleet.ammo_bank
                    // } else {
                    //     &fleet.cargo_hold
                    // };

                    // match fleet_state {
                    //     FleetState::StarbaseLoadingBay(state) => {
                    //         let ixs = ixs::cargo::deposit_to_fleet(
                    //             &sage_program,
                    //             &cargo_program,
                    //             (fleet_id, &fleet),
                    //             (&game_id, &game),
                    //             &state.starbase,
                    //             cargo_pod_to,
                    //             mint,
                    //             *amount,
                    //         )?;

                    //         Some(ixs)
                    //     }
                    //     _ => {
                    //         println!("Fleet is not docked at a starbase");
                    //         None
                    //     }
                    // }
                }
                Actions::CargoWithdraw {
                    fleet_id,
                    mint,
                    amount,
                } => {
                    todo!("Actions::CargoWithdraw");
                    // HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp

                    // let (fleet, fleet_state) =
                    //     derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    // match fleet_state {
                    //     FleetState::StarbaseLoadingBay(state) => {
                    //         let ixs = ixs::cargo::withdraw_from_fleet(
                    //             &sage_program,
                    //             &cargo_program,
                    //             (fleet_id, &fleet),
                    //             (&game_id, &game),
                    //             &state.starbase,
                    //             mint,
                    //             *amount,
                    //         )?;

                    //         Some(ixs)
                    //     }
                    //     _ => {
                    //         println!("Fleet is not docked at a starbase");
                    //         None
                    //     }
                    // }
                }
                Actions::StarbaseDock { fleet_id } => {
                    let (fleet_id, FleetWithState(fleet, state)) =
                        SageBasedGameHandler::get_fleet_with_state(&sage_program, &fleet_id)
                            .await?;

                    let sector = match &state {
                        FleetState::Idle(idle) => {
                            let sector = idle.sector;
                            sector
                        }
                        _ => {
                            println!("Fleet is idle at a starbase");
                            return Ok(());
                        }
                    };

                    let ix = ixs::dock_to_starbase(
                        &sage_program,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        sector,
                    );
                    Some(ix)
                }
                Actions::StarbaseUndock { fleet_id } => {
                    let (fleet_id, FleetWithState(fleet, state)) =
                        SageBasedGameHandler::get_fleet_with_state(&sage_program, &fleet_id)
                            .await?;

                    let starbase = match &state {
                        FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                            let starbase = starbase_loading_bay.starbase;
                            starbase
                        }
                        _ => {
                            println!("Fleet is not dockeed at a starbase");
                            return Ok(());
                        }
                    };

                    let ix = ixs::undock_from_starbase(
                        &sage_program,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        &starbase,
                    );
                    Some(ix)
                }
                Actions::StartMining {
                    fleet_id,
                    mine_item,
                } => {
                    let (fleet_id, FleetWithState(fleet, state)) =
                        SageBasedGameHandler::get_fleet_with_state(&sage_program, &fleet_id)
                            .await?;

                    match &state {
                        FleetState::Idle(idle) => {
                            let sector = idle.sector;
                            sector
                        }
                        _ => {
                            println!("Fleet is not idle");
                            return Ok(());
                        }
                    };

                    todo!();
                    // let ix = ixs::start_mining_asteroid(&sage_program, (&game_id, &game), (&fleet_id, &fleet), mine_item, resource, planet, sector);
                    // Some(ix);
                }
                Actions::StopMining { fleet_id } => {
                    todo!();

                    // let (fleet, fleet_state) =
                    //     derive::fleet_account_with_state(&sage_program, &fleet_id)?;

                    // let ixs = ixs::mine::stop_mining_asteroid(
                    //     &sage_program,
                    //     (fleet_id, (&fleet, &fleet_state)),
                    //     (&game_id, &game),
                    // )?;

                    // Some(ixs)
                }
                Actions::Warp {
                    fleet_id,
                    x_coord,
                    y_coord,
                } => {
                    let (fleet_id, FleetWithState(fleet, _state)) =
                        SageBasedGameHandler::get_fleet_with_state(&sage_program, &fleet_id)
                            .await?;

                    let ix = ixs::warp_to_coordinate(
                        &sage_program,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                        [*x_coord, *y_coord],
                    );
                    Some(ix)
                }
                Actions::WarpExit { fleet_id } => {
                    let (fleet_id, FleetWithState(fleet, state)) =
                        SageBasedGameHandler::get_fleet_with_state(&sage_program, &fleet_id)
                            .await?;

                    let ix = ixs::warp_ready_to_exit(
                        &sage_program,
                        (&game_id, &game),
                        (&fleet_id, &fleet),
                    );
                    Some(ix)
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

            if let Some(ix) = ixs {
                let instructions = vec![ix];

                let simulated = SageBasedGameHandler::simulate_transaction(
                    &sage_program,
                    &instructions,
                    &vec![&payer],
                )
                .await?;
                dbg!(simulated);

                // let result = SageBasedGameHandler::simulate_and_send_transaction(&sage_program, &payer, &instructions).await;
                // dbg!(result);
            }
        }
        Commands::Find(find) => match find {
            Find::Games => {
                let games = sage_program.accounts::<state::Game>(vec![]).await?;

                let mut table = comfy_table::Table::new();
                table.set_header(vec!["Game ID", "Version", "Mints"]);

                for (pubkey, game) in games {
                    // dbg!(&game.points);
                    table.add_row(vec![
                        pubkey.to_string(),
                        game.version.to_string(),
                        format!("{:#?}", game.mints),
                    ]);
                }

                println!("{table}");
            }
            Find::Fleet { fleet_name } => {
                let (pubkey, _) =
                    addr::fleet_address(&game_id, &player_profile_id, fleet_name.as_str());
                let (pubkey, FleetWithState(fleet, _state)) =
                    SageBasedGameHandler::get_fleet_with_state(&sage_program, &pubkey).await?;
                dbg!((pubkey, fleet));
            }
            Find::PlayerProfile => {
                unimplemented!("Find::PlayerProfile");

                // let profile_accounts =
                //     derive_profile_accounts(&player_profile_program, &payer.pubkey())?;

                // let mut table = comfy_table::Table::new();
                // table.set_header(vec![
                //     "Profile ID",
                //     "Version",
                //     "Auth Key Count",
                //     "Key Threshold",
                // ]);

                // for (pubkey, profile) in profile_accounts {
                //     table.add_row(vec![
                //         pubkey.to_string(),
                //         profile.0.version.to_string(),
                //         profile.0.auth_key_count.to_string(),
                //         profile.0.key_threshold.to_string(),
                //     ]);
                // }

                // println!("{table}");
            }
            Find::Planets { x, y } => {
                let planets =
                    filter::planets_by_game_and_coords(&sage_program, &game_id, [*x, *y]).await?;
                dbg!(&planets);

                // let planets = derive::planet_accounts(&sage_program, &game_id, [*x, *y])?;
                // let (pubkey, planet) = planets
                //     .into_iter()
                //     .find(|(_, planet)| planet.num_resources >= 1)
                //     .expect("planet with resources");

                // dbg!(&planet);
                // let resources = derive::resource_accounts(&sage_program, &game_id, &pubkey)?;

                // for (pubkey, resource) in resources {
                //     dbg!(pubkey);
                //     dbg!(&resource);

                //     let mine_item = derive::mine_item_account(&sage_program, &resource.mine_item)?;
                //     let name = mine_item.name();

                //     dbg!(name);
                //     dbg!(mine_item);
                // }
            }
            Find::PointsModifiers => {
                unimplemented!("Find::PointsModifiers");
                // let accounts = points_program.accounts::<staratlas_sage_sdk::programs::staratlas_points::state::PointsModifier>(vec![])?;

                // for (pubkey, account) in accounts {
                //     println!("{:#?}", (pubkey, account.version, account.point_category));
                // }
            }
            _ => {
                todo!()
            }
        },
        Commands::Show(show) => match show {
            Show::AllFleets => {
                let fleets = filter::fleets_by_game_and_player_profile(
                    &sage_program,
                    &game_id,
                    &player_profile_id,
                )
                .await?;

                let mut table = comfy_table::Table::new();
                table.set_header(vec![
                    "Fleet ID",
                    "Fleet Label",
                    "Ship Counts",
                    "Movement Stats",
                    "Cargo Stats",
                    "Misc Stats",
                ]);

                for (pubkey, fleet) in fleets {
                    table.add_row(vec![
                        pubkey.to_string(),
                        fleet.name().to_string(),
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
                let (_, FleetWithState(fleet, state)) =
                    SageBasedGameHandler::get_fleet_with_state(&sage_program, &fleet_id).await?;

                if *with_state {
                    println!("{:#?}", (fleet, state));
                } else {
                    println!("{:#?}", fleet);
                }
            }
            Show::Game { game_id } => {
                let (_, game) = SageBasedGameHandler::get_game(&sage_program, &game_id).await?;
                println!("{:#?}", game);
            }
        },
    }

    Ok(())
}
