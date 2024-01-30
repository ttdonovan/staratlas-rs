use anchor_client::{
    solana_sdk::{
        account::Account,
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
    },
    Client, Cluster,
};
use anchor_lang::{prelude::AccountMeta, AnchorDeserialize, Id};
use clap::{Parser, Subcommand};
use staratlas_player_profile_sdk::{
    accounts,
    instruction,
    program::PlayerProfile,
    typedefs,
    utils::{derive_profile_accounts, get_profile_accounts},
};

use std::rc::Rc;
use std::str::FromStr;

/// Star Atlas: Player Profile CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    config: Config,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Default, Parser)]
struct Config {
    /// RPC URL for the Solana cluster.
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    pub cluster: Option<Cluster>,
    /// Wallet keypair to use.
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    wallet: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    // Add a key to the Profile's permissioned accounts
    #[command(arg_required_else_help = true)]
    AddKey {
        new_key: String,
    },
    // Remove a key from the Profile's permissioned accounts
    RemoveKey {
        old_key: String,
    },
    // Show the Profile's permissioned accounts
    ShowPermissionedAccounts,
    // Show Profile details
    ShowProfile,
}

fn default_keypair() -> Keypair {
    read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Requires a keypair file")
}

// Bitshift taken from @staratlas/sage permissions.ts
fn build_permissions(input: [[bool; 8]; 3]) -> [u8; 8] {
    let mut out = [0u8; 8];
    for i in 0..3 {
        for j in 0..8 {
            if input[i][j] {
                out[i] |= 1 << j;
            }
        }
    }
    out
}

fn derive_permissioned_profile_keys(
    account: &Account,
) -> anyhow::Result<Vec<typedefs::ProfileKey>> {
    let mut profile_keys = vec![];

    // first 30 bytes are the profile and each subsequent 80 bytes is a permissioned account
    let permissioned_data = account.data[30..].chunks_exact(80);
    for data in permissioned_data {
        let profile_key =
            typedefs::ProfileKey::try_from_slice(&mut &data[..])?;
        profile_keys.push(profile_key)
    }
    Ok(profile_keys)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let payer = match cli.config.wallet {
        Some(wallet) => read_keypair_file(wallet).expect("Requires a keypair file"),
        None => default_keypair(),
    };

    let url = match cli.config.cluster {
        Some(cluster) => cluster,
        None => Cluster::Devnet,
    };

    let client = Client::new_with_options(
        url,
        Rc::new(Keypair::from_bytes(&payer.to_bytes())?),
        CommitmentConfig::confirmed(),
    );

    let player_profile = payer.pubkey();

    let sage_program_id = Pubkey::from_str("SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6")?;
    let profile_program = client.program(PlayerProfile::id())?;

    match &cli.command {
        Commands::AddKey { new_key } => {
            let profile_accounts = get_profile_accounts(&profile_program, &player_profile)?;
            let (profile_pubkey, _account) = &profile_accounts[0];

            let new_key = Pubkey::from_str(new_key)?;
            // Waiting on documentation explaining the permissions. Ideally we would request only necessary permissions. For now, we're requesting all SAGE permissions except 'rentFleet'
            let permissions = build_permissions([
                [true, true, true, true, true, false, true, true],
                [true, false, true, true, true, false, true, true],
                [true, true, true, true, true, true, true, true],
            ]);

            let ix = instruction::AddKeys {
                _key_add_index: 0,
                _key_permissions_index: 0,
                _keys_to_add: vec![typedefs::AddKeyInput {
                    scope: sage_program_id,
                    expire_time: -1,
                    permissions,
                }],
            };

            let builder = profile_program
                .request()
                .accounts(accounts::AddKeys {
                    funder: payer.pubkey(),
                    key: payer.pubkey(),
                    profile: *profile_pubkey,
                    system_program: system_program::id(),
                })
                .accounts(AccountMeta {
                    pubkey: new_key,
                    is_signer: false,
                    is_writable: false,
                })
                .args(ix);

            // let instructions = builder.instructions();
            // dbg!(instructions);

            let signature = builder.send()?;
            println!("{}", signature);
        }
        Commands::RemoveKey { old_key } => {
            let old_key = Pubkey::from_str(old_key)?;

            let profile_accounts = get_profile_accounts(&profile_program, &player_profile)?;
            let (profile_pubkey, account) = &profile_accounts[0];
            let profile_keys = derive_permissioned_profile_keys(account)?;

            if let Some(idx) = profile_keys
                .iter()
                .position(&|k: &typedefs::ProfileKey| &k.key == &old_key)
            {
                if idx != 0 {
                    let ix = instruction::RemoveKeys {
                        _key_index: 0,
                        _keys_to_remove: [idx as u16, (idx + 1) as u16],
                    };

                    let builder = profile_program
                        .request()
                        .accounts(accounts::RemoveKeys {
                            funder: payer.pubkey(),
                            key: payer.pubkey(),
                            profile: *profile_pubkey,
                            system_program: system_program::id(),
                        })
                        .args(ix);

                    // let instructions = builder.instructions();
                    // dbg!(instructions);

                    let signature = builder.send()?;
                    println!("{}", signature);
                } else {
                    println!("Cannot remove the first key");
                }
            } else {
                println!("Key not found");
            }
        }
        Commands::ShowPermissionedAccounts => {
            let profile_accounts = get_profile_accounts(&profile_program, &player_profile)?;
            let (_profile_pubkey, account) = &profile_accounts[0];
            let profile_keys = derive_permissioned_profile_keys(account)?;

            println!("{:?}", profile_keys);
        }
        Commands::ShowProfile => {
            let profile_accounts = derive_profile_accounts(&profile_program, &player_profile)?;
            let (profile_pubkey, profile) = &profile_accounts[0];

            println!("{:?}", profile_pubkey);
            println!("{:?}", &profile);
        }
    }

    Ok(())
}
