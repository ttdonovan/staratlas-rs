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
    fleets::{
        derive_fleet_account, derive_fleet_account_with_state, derive_fleet_accounts,
        derive_fleet_address, get_fleet_account,
    },
    games::{derive_game_account, derive_game_accounts, derive_game_state_account},
    program::Sage,
};

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
    /// Sage Game State's Pubkey
    #[clap(long = "sage.game_state_id", env = "SAGE_GAME_STATE_ID")]
    game_state_id: Option<Pubkey>,
    /// Sage Player Profile's Pubkey
    #[clap(long = "sage.profile_id", env = "SAGE_PROFILE_ID")]
    profile_id: Option<Pubkey>,
}

#[derive(Subcommand)]
enum Commands {
    ShowFleet {
        /// Fleet's Pubkey
        #[arg(long)]
        fleet_id: Option<Pubkey>,
        /// Fleet's Label
        #[arg(long)]
        fleet_label: Option<String>,
        /// Show Fleet's State (default: false)
        #[arg(long, default_value_t = false)]
        with_state: bool,
    },
    ShowFleets,
    ShowGame,
    ShowGames,
    ShowGameState,
    ShowPlayerProfile,
}

fn default_keypair() -> Keypair {
    read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Requires a keypair file")
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
        Commands::ShowFleet {
            fleet_id,
            fleet_label,
            with_state,
        } => {
            if let Some(fleet) = match (fleet_id.as_ref(), fleet_label.as_ref()) {
                (Some(fleet_id), _) => {
                    if *with_state {
                        let (fleet, fleet_state) =
                            derive_fleet_account_with_state(&sage_program, &fleet_id)?;
                        Some((fleet, Some(fleet_state)))
                    } else {
                        let fleet = derive_fleet_account(&sage_program, &fleet_id)?;
                        Some((fleet, None))
                    }
                }
                (_, Some(fleet_label)) => {
                    let game_id = &cli
                        .sage_config
                        .game_id
                        .expect("Requires --sage.game_state_id <GAME_STATE_ID>");

                    let profile_id = &cli
                        .sage_config
                        .profile_id
                        .expect("Requires --sage.profile_id <PROFILE_ID>");

                    let (fleet_pubkey, _) =
                        derive_fleet_address(&game_id, &profile_id, fleet_label.as_str());

                    if *with_state {
                        let (fleet, fleet_state) =
                            derive_fleet_account_with_state(&sage_program, &fleet_pubkey)?;
                        Some((fleet, Some(fleet_state)))
                    } else {
                        let fleet = derive_fleet_account(&sage_program, &fleet_pubkey)?;
                        Some((fleet, None))
                    }
                }
                _ => {
                    println!(
                        "Requires --fleet_pubkey <FLEET_PUBKEY> or --fleet_label <FLEET_LABEL>"
                    );
                    None
                }
            } {
                println!("{:#?}", fleet);
            };
        }
        Commands::ShowFleets => {
            let game_id = &cli
                .sage_config
                .game_id
                .expect("Requires --sage.game_state_id <GAME_STATE_ID>");

            let profile_id = &cli
                .sage_config
                .profile_id
                .expect("Requires --sage.profile_id <PROFILE_ID>");

            let fleet_accounts = derive_fleet_accounts(&sage_program, game_id, profile_id)?;

            println!("{:#?}", fleet_accounts);
        }
        Commands::ShowGame => {
            let game = derive_game_account(
                &sage_program,
                &cli.sage_config
                    .game_id
                    .expect("Requires --sage.game_state_id <GAME_STATE_ID>"),
            )?;

            println!("{:#?}", game);
        }

        Commands::ShowGames => {
            let game_accounts = derive_game_accounts(&sage_program)?;

            println!("{:#?}", game_accounts);
        }
        Commands::ShowGameState => {
            let game_state = derive_game_state_account(
                &sage_program,
                &cli.sage_config
                    .game_state_id
                    .expect("Requires --sage.game_state_id <GAME_STATE_ID>"),
            )?;

            println!("{:#?}", game_state);
        }
        Commands::ShowPlayerProfile => {
            let profile_accounts =
                derive_profile_accounts(&player_profile_program, &payer.pubkey())?;

            println!("{:#?}", profile_accounts);
        }
    }

    Ok(())
}
