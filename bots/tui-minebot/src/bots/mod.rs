use anchor_client::solana_sdk::{pubkey::Pubkey, signature::Signature};

use std::time::Duration;

use crate::{labs, sage, time};

mod bot;
mod calc;
pub(crate) mod process;
pub(crate) mod run;

pub use bot::*;
pub(crate) use calc::*;

pub fn ui_masked_pubkey(pubkey: &Pubkey) -> String {
    let id = pubkey.to_string();
    format!("{}...{}", &id[..4], &id[40..])
}
