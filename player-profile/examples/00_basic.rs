use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::signer::null_signer::NullSigner,
    Client, Cluster,
};
use anchor_lang::Id;

use std::str::FromStr;

use staratlas_player_profile::{
    program::PlayerProfile,
    state::{PlayerName, Profile},
};
use staratlas_utils_config as config;

fn main() -> anyhow::Result<()> {
    // Load the configuration from the environment variables
    let config = config::load_from_env();

    // Get the Player Profile and Solana RPC URL from the configuration
    let player_profile = config
        .pubkey_player_profile
        .ok_or(anyhow::anyhow!("Player Profile not found"))?;
    let player_profile = Pubkey::from_str(&player_profile)?;

    let rpc_url = config
        .solana_rpc_url
        .ok_or(anyhow::anyhow!("RPC URL not found"))?;

    // Create a wallet (from player profile) and payer
    let wallet = player_profile.clone();
    let payer = NullSigner::new(&wallet);

    // Create a new Anchor client
    let client = Client::new(Cluster::Custom(rpc_url.clone(), rpc_url), &payer);

    // Get the program ID for the PlayerProfile program
    let program_id = PlayerProfile::id();
    dbg!(&program_id);

    let program = client.program(program_id);

    dbg!(&player_profile);

    // Get the Profile accounts from the PlayerProfile program
    let profile_accounts = program.accounts::<Profile>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(30, &player_profile.to_bytes()),
    )])?;

    // Iterate over the Profile accounts and print out the profile information
    for (pubkey, profile) in profile_accounts {
        dbg!(&pubkey);
        dbg!(&profile.version);
        dbg!(&profile.auth_key_count);
        dbg!(&profile.key_threshold);
        dbg!(&profile.next_seq_id);
        dbg!(&profile.created_at);

        // Lookup the PlayerName account for the PlayerProfile program
        let (name_account, _bump) =
            Pubkey::find_program_address(&[b"player_name", &pubkey.to_bytes()], &program_id);
        dbg!(&name_account);

        // Get the PlayerName account from the PlayerProfile program
        let player_name = program.account::<PlayerName>(name_account)?;
        dbg!(&player_name.version);
        dbg!(&player_name.profile);
        dbg!(&player_name.bump);

        assert_eq!(&pubkey, &player_name.profile);
    }

    Ok(())
}
