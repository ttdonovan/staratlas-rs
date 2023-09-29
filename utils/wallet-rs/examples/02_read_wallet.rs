use solana_sdk::signature::Signer;
use staratlas_utils_wallet::open_wallet_from_file_with_password_prompt;

fn main() -> anyhow::Result<()> {
    let wallet = open_wallet_from_file_with_password_prompt("tmp/wallet.enc")?;

    let keypair = wallet
        .get_keypair("main")
        .expect("wallet keypair not found");

    let pubkey = keypair.pubkey();
    dbg!(&pubkey);

    Ok(())
}
