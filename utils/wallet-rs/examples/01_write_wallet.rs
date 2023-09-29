use staratlas_utils_wallet::create_wallet_from_private_key_and_password_prompt;

fn main() -> anyhow::Result<()> {
    create_wallet_from_private_key_and_password_prompt("main", "tmp/wallet.enc")?;

    Ok(())
}
