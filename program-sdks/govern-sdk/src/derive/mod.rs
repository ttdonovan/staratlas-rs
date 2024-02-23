use anchor_client::{
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, RpcFilterType},
    },
    solana_sdk::{
        account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signer,
    },
    Program,
};
use solana_account_decoder::UiAccountEncoding;

use std::ops::Deref;

pub mod atlas;
pub mod locked_voter;
pub mod proxy_rewarder;
