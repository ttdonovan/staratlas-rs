use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::signer::keypair::{keypair_from_seed, Keypair};

use std::collections::HashMap;
use std::fmt;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Wallet {
    inner: HashMap<String, [u8; 64]>,
}

/// A collection of keypairs identified by aliases.
impl Wallet {
    /// Creates a new, empty `Wallet` instance.
    pub fn new() -> Self {
        Wallet {
            inner: HashMap::new(),
        }
    }

    /// Returns the `Keypair` associated with the given alias, if it exists.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias of the keypair to retrieve.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_utils_wallet::Wallet;
    ///
    /// let mut wallet = Wallet::new();
    /// let keypair = solana_sdk::signer::keypair::Keypair::new();
    /// wallet.insert("my-keypair", keypair.to_bytes());
    ///
    /// let retrieved_keypair = wallet.get_keypair("my-keypair").unwrap();
    /// assert_eq!(retrieved_keypair.to_bytes(), keypair.to_bytes());
    /// ```
    pub fn get_keypair(&self, alias: &str) -> Option<Keypair> {
        let seed = self.get_seed(alias)?;
        let keypair = keypair_from_seed(seed).unwrap();
        Some(keypair)
    }

    /// Returns the seed associated with the given alias, if it exists.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias of the seed to retrieve.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_utils_wallet::Wallet;
    ///
    /// let mut wallet = Wallet::new();
    /// let keypair = solana_sdk::signer::keypair::Keypair::new();
    /// wallet.insert("my-keypair", keypair.to_bytes());
    ///
    /// let retrieved_seed = wallet.get_seed("my-keypair").unwrap();
    /// assert_eq!(retrieved_seed, keypair.to_bytes());
    /// ```
    pub fn get_seed(&self, alias: &str) -> Option<&[u8; 64]> {
        self.inner.get(alias)
    }

    /// Inserts a new keypair into the `Wallet` with the given alias and private key.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias to associate with the new keypair.
    /// * `private_key` - The private key of the keypair, encoded as a base58 string.
    ///
    /// # Examples
    ///
    /// ```
    /// use staratlas_utils_wallet::Wallet;
    ///
    /// let mut wallet = Wallet::new();
    /// let private_key = "3vJZJ5JzJ5JzJ5JzJ5JzJ5JzJ5JzJ5JzJ5JzJ5JzJ5Jz".to_string();
    /// wallet.insert("my-keypair", private_key);
    ///
    /// let retrieved_keypair = wallet.get_keypair("my-keypair").unwrap();
    /// assert_eq!(retrieved_keypair.to_bytes(), [0x3c, 0x3d, 0x3e, ...]);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the private key cannot be decoded from the base58 string.
    pub fn insert<S>(&mut self, alias: S, private_key: String)
    where
        S: Into<String>,
    {
        let keypair = Keypair::from_base58_string(&private_key);
        self.inner.insert(alias.into(), keypair.to_bytes());
    }
}

impl fmt::Debug for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wallet").finish()
    }
}
