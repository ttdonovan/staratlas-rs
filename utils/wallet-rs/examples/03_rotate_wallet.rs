use staratlas_utils_wallet::{
    open_wallet_from_file_with_password_prompt, rotate_wallet_password_with_prompt,
};

fn main() -> anyhow::Result<()> {
    let wallet = open_wallet_from_file_with_password_prompt("tmp/wallet.enc")?;
    rotate_wallet_password_with_prompt(wallet, "tmp/wallet.enc")?;

    Ok(())
}
