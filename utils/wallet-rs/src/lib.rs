use borsh::{BorshDeserialize, BorshSerialize};
use cocoon::Cocoon;

use std::fs::File;
use std::path::Path;

pub(crate) mod prompts;
pub mod wallet;

use wallet::Wallet;

fn create_wallet_file<P>(wallet: Wallet, password: String, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = File::create(path)?;
    let encoded = wallet.try_to_vec()?;

    let cocoon = Cocoon::new(password.as_bytes());
    let _ = cocoon
        .dump(encoded, &mut file)
        .expect("Failed to write wallet file");

    Ok(())
}

fn open_wallet_file<P>(password: String, path: P) -> anyhow::Result<Wallet>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let cocoon = Cocoon::parse_only(password.as_bytes());

    let data = cocoon.parse(&mut file).expect("Failed to open wallet file");
    let wallet = Wallet::try_from_slice(&data)?;

    Ok(wallet)
}

/// Creates a new `Wallet` instance from a private key and password entered by the user.
///
/// # Arguments
///
/// * `alias` - The alias to associate with the new keypair.
/// * `path` - The path to the file where the new `Wallet` will be stored.
///
/// # Examples
///
/// ```
/// use staratlas_utils_wallet::create_wallet_from_private_key_and_password_prompt;
///
/// let wallet = create_wallet_from_private_key_and_password_prompt("my-keypair", "/path/to/wallet").unwrap();
/// ```
pub fn create_wallet_from_private_key_and_password_prompt<P>(
    alias: &str,
    path: P,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let (private_key, password) = prompts::prompt_for_private_key_and_password()?;

    let mut wallet = Wallet::new();
    wallet.insert(alias, private_key);

    create_wallet_file(wallet, password, path)?;

    Ok(())
}

/// Opens a `Wallet` instance from a file encrypted with a password entered by the user.
///
/// # Arguments
///
/// * `path` - The path to the file containing the encrypted wallet data.
///
/// # Examples
///
/// ```
/// use staratlas_utils_wallet::open_wallet_from_file_with_password_prompt;
///
/// let wallet = open_wallet_from_file_with_password_prompt("/path/to/wallet").unwrap();
/// ```
pub fn open_wallet_from_file_with_password_prompt<P>(path: P) -> anyhow::Result<Wallet>
where
    P: AsRef<Path>,
{
    let password = prompts::prompt_for_password()?;
    let wallet = open_wallet_file(password, path)?;

    Ok(wallet)
}

pub fn rotate_wallet_password_with_prompt<P>(wallet: Wallet, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let password = prompts::prompt_for_new_password()?;
    create_wallet_file(wallet, password, path)?;

    Ok(())
}
