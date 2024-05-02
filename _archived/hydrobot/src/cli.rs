use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
    },
    Client, Cluster,
};
use clap::Parser;

use std::rc::Rc;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    provider_config: ProviderConfig,
    #[clap(flatten)]
    sage_config: SageConfig,
    /// Autoplay enabled (default: false)
    #[arg(long, default_value_t = false)]
    with_autoplay: bool,
}

#[derive(Default, Parser)]
struct ProviderConfig {
    /// RPC URL for the Solana cluster.
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    cluster: Option<Cluster>,
    /// Wallet keypair to use.
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    wallet: Option<String>,
}

#[derive(Default, Parser)]
struct SageConfig {
    /// Sage Game's Pubkey
    #[clap(long = "sage.game_id", env = "SAGE_GAME_ID")]
    game_id: Option<Pubkey>,
    /// Sage Fleet's Pubkey
    #[clap(long = "sage.fleet_id")]
    fleet_id: Vec<Pubkey>,
}

fn default_keypair() -> Keypair {
    read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Requires a keypair file")
}

impl Cli {
    pub fn autoplay_enabled(&self) -> bool {
        self.with_autoplay
    }
}

pub fn init_payer_and_client(cli: &Cli) -> anyhow::Result<(Keypair, Client<Rc<Keypair>>)> {
    let payer = match &cli.provider_config.wallet {
        Some(wallet) => read_keypair_file(wallet).expect("Requires a keypair file"),
        None => default_keypair(),
    };

    let url = match &cli.provider_config.cluster {
        Some(cluster) => cluster.clone(),
        None => Cluster::Devnet,
    };

    let client = Client::new_with_options(
        url,
        Rc::new(Keypair::from_bytes(&payer.to_bytes())?),
        CommitmentConfig::confirmed(),
    );

    Ok((payer, client))
}

pub fn init_sage_config(cli: &Cli) -> (Pubkey, Vec<Pubkey>) {
    let game_id = cli
        .sage_config
        .game_id
        .expect("Requires --sage.game_state_id <GAME_STATE_ID>");

    let fleet_ids = cli.sage_config.fleet_id.clone();

    (game_id, fleet_ids)
}
