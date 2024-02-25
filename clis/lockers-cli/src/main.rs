use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster,
};
use clap::{Parser, Subcommand};

use staratlas_govern_sdk::{
    derive,
    programs::{
        staratlas_atlas_staking::ID as ATLAS_STAKING_ID,
        staratlas_locked_voter::ID as LOCKED_VOTER_ID,
        staratlas_proxy_rewarder::ID as PROXY_REWARDER_ID,
    },
};

use std::rc::Rc;

/// Star Atlas: Locker CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    provider_config: ProviderConfig,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Default, Parser)]
struct ProviderConfig {
    /// RPC URL for the Solana cluster
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    pub cluster: Option<Cluster>,
    /// Wallet keypair to use
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    wallet: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Find(Find),
    #[command(subcommand)]
    Show(Show),
}

#[derive(Subcommand)]
enum Find {
    AtlasStaking,
    /// Locked Voter: Escrow Account
    Escrow {
        /// Proxy Account address (pubkey)
        owner_id: Pubkey,
    },
    /// Proxy Rewarder: Proxy Account
    Proxy {
        /// Wallet address (pubkey)
        owner_id: Pubkey,
    },
    /// Proxy Rewarder: Proxy Escrow Account
    ProxyEscrow {
        /// Wallet address (pubkey)
        escrow_owner_id: Pubkey,
    },
    /// Proxy Rewarder: Registered Locker Account
    RegisteredLocker {
        /// Locker Account address (pubkey)
        locker_id: Pubkey,
    },
}

#[derive(Subcommand)]
enum Show {
    /// Locked Voter: Escrow Account
    Escrow {
        /// Account address
        pubkey: Pubkey,
    },
    /// Locked Voter: Locker Account
    Locker {
        /// Account address
        pubkey: Pubkey,
    },
    /// Locked Voter: Locker Whitelist Entry Account
    LockerWhitelistEntry {
        /// Account address
        pubkey: Pubkey,
    },
    /// Proxy Rewarder: Proxy Account
    Proxy {
        /// Account address
        pubkey: Pubkey,
    },
    /// Proxy Rewarder: Proxy Escrow Account
    ProxyEscrow {
        /// Account address
        pubkey: Pubkey,
    },
    /// Proxy Rewarder: Registered Locker Account
    RegisteredLocker {
        /// Account address
        pubkey: Pubkey,
    },
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

    let atlas_staking_program = client.program(ATLAS_STAKING_ID)?;
    let locked_voter_program = client.program(LOCKED_VOTER_ID)?;
    let proxy_rewarder_program = client.program(PROXY_REWARDER_ID)?;

    match &cli.command {
        Commands::Find(find) => match find {
            Find::AtlasStaking => {
                let staking_accounts =
                    derive::atlas::staking_accounts(&atlas_staking_program, &payer.pubkey())?;
                println!("{:#?}", staking_accounts);
            }
            Find::Escrow { owner_id } => {
                let escrow_accounts =
                    derive::locked_voter::escrow_accounts(&locked_voter_program, &owner_id)?;
                println!("{:#?}", escrow_accounts);
            }
            Find::Proxy { owner_id } => {
                let proxy_accounts =
                    derive::proxy_rewarder::proxy_accounts(&proxy_rewarder_program, &owner_id)?;
                println!("{:#?}", proxy_accounts);
            }
            Find::ProxyEscrow { escrow_owner_id } => {
                let proxy_escrow_accounts = derive::proxy_rewarder::proxy_escrow_accounts(
                    &proxy_rewarder_program,
                    &escrow_owner_id,
                )?;
                println!("{:#?}", proxy_escrow_accounts);
            }
            Find::RegisteredLocker { locker_id } => {
                let registered_locker_accounts =
                    derive::proxy_rewarder::registered_locker_accounts(
                        &proxy_rewarder_program,
                        &locker_id,
                    )?;
                println!("{:#?}", registered_locker_accounts);
            }
        },
        Commands::Show(show) => match show {
            Show::Escrow { pubkey } => {
                let account = derive::locked_voter::escrow_account(&locked_voter_program, &pubkey)?;
                println!("{:#?}", account);
            }
            Show::Locker { pubkey } => {
                let account = derive::locked_voter::locker_account(&locked_voter_program, &pubkey)?;
                println!("{:#?}", account);
            }
            Show::LockerWhitelistEntry { pubkey } => {
                let account = derive::locked_voter::locker_whitelist_entry_account(
                    &locked_voter_program,
                    &pubkey,
                )?;
                println!("{:#?}", account);
            }
            Show::Proxy { pubkey } => {
                let account =
                    derive::proxy_rewarder::proxy_account(&proxy_rewarder_program, &pubkey)?;
                println!("{:#?}", account);
            }
            Show::ProxyEscrow { pubkey } => {
                let account =
                    derive::proxy_rewarder::proxy_escrow_account(&proxy_rewarder_program, &pubkey)?;
                println!("{:#?}", account);
            }
            Show::RegisteredLocker { pubkey } => {
                let account = derive::proxy_rewarder::registered_locker_account(
                    &proxy_rewarder_program,
                    &pubkey,
                )?;
                println!("{:#?}", account);
            }
        },
    }

    Ok(())
}
