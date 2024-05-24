use anchor_client::{anchor_lang::prelude::Pubkey, Cluster};
use clap::{Parser, Subcommand};

/// Star Atlas: Sage CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub provider_config: ProviderConfig,
    #[clap(flatten)]
    pub sage_config: SageConfig,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Default, Parser)]
pub struct ProviderConfig {
    /// RPC URL for the Solana cluster.
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    pub cluster: Option<Cluster>,
    /// Wallet keypair to use.
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    pub wallet: Option<String>,
}

#[derive(Debug, Default, Parser)]
pub struct SageConfig {
    /// Sage Game's Pubkey
    #[clap(long = "sage.game_id", env = "SAGE_GAME_ID")]
    pub game_id: Option<Pubkey>,
    /// Sage Player Profile's Pubkey
    #[clap(long = "sage.profile_id", env = "SAGE_PROFILE_ID")]
    pub profile_id: Option<Pubkey>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Actions(Actions),
    #[command(subcommand)]
    Find(Find),
    #[command(subcommand)]
    Show(Show),
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Actions {
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
pub enum Find {
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
pub enum Show {
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
