use anchor_client::{solana_sdk::signer::Signer, Client, Cluster};
use anchor_lang::Id;

use staratlas_player_profile::{
    instruction, ix_accounts, program::PlayerProfile, typedefs::AddKeyInput,
};

use staratlas_utils_config as config;
use staratlas_utils_wallet as wallet;

fn main() -> anyhow::Result<()> {
    let config = config::load_from_env();

    let rpc_url = config
        .solana_rpc_url
        .ok_or(anyhow::anyhow!("RPC URL not found"))?;

    let alais = config
        .wallet_alias
        .ok_or(anyhow::anyhow!("Wallet alias not found"))?;

    let password = config
        .wallet_password
        .ok_or(anyhow::anyhow!("Wallet password not found"))?;

    let wallet = wallet::open_wallet_file(password, "tmp/wallet.enc")?;

    let keypair = wallet.get_keypair(&alais).expect("Keypair not found");
    let pubkey = keypair.pubkey();
    dbg!(&pubkey);

    let client = Client::new(Cluster::Custom(rpc_url.clone(), rpc_url), &keypair);

    let program_id = PlayerProfile::id();
    let program = client.program(program_id);

    let key_input = AddKeyInput {
        scope: keypair.pubkey(),
        expire_time: 0,
        permissions: [1, 1, 0, 0, 0, 0, 0, 0],
    };

    let create_profile = instruction::CreateProfile {
        _key_permissions: vec![key_input],
        _key_threshold: 1,
    };

    dbg!("here...");

    Ok(())
}
