use dotenv::dotenv;

use std::fmt;

const ENV_PUBKEY_PLAYER_PROFILE: &str = "PUBKEY_PLAYER_PROFILE";
const ENV_SOLANA_RPC_URL: &str = "SOLANA_RPC_URL";
const ENV_WALLET_PASSWORD: &str = "WALLET_PASSWORD";

pub fn load_from_env() -> Config {
    dotenv().ok();
    Config::from_env()
}

pub struct Config {
    pub pubkey_player_profile: Option<String>,
    pub solana_rpc_url: Option<String>,
    pub wallet_password: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let pubkey_player_profile = dotenv::var(ENV_PUBKEY_PLAYER_PROFILE).ok();
        let solana_rpc_url = dotenv::var(ENV_SOLANA_RPC_URL).ok();
        let wallet_password = dotenv::var(ENV_WALLET_PASSWORD).ok();

        Config {
            pubkey_player_profile,
            solana_rpc_url,
            wallet_password,
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked_password = match &self.wallet_password {
            Some(_) => Some("********"),
            None => None,
        };

        f.debug_struct("Config")
            .field("solana_rpc_url", &self.solana_rpc_url)
            .field("wallet_password", &masked_password)
            .finish()
    }
}

// cargo test -p staratlas-utils-config -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = load_from_env();
        dbg!(&config);

        assert!(config.solana_rpc_url.is_some());
        assert!(config.wallet_password.is_some());
    }
}
