use anchor_lang::prelude::Pubkey;
use borsh::BorshDeserialize;

pub mod derive;
pub mod programs;

#[derive(Debug, BorshDeserialize)]
pub struct Escrow {
    pub locker: Pubkey,
    pub owner: Pubkey,
    pub bump: u8,
    pub tokens: Pubkey,
    pub amount: u64,
    pub escrowed_started_ts: i64,
    pub escrowed_ends_ts: i64,
    pub vote_delegate: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct Locker {
    pub base: Pubkey,
    pub bump: u8,
    pub token_mint: Pubkey,
    pub locked_supply: u64,
    pub governor: Pubkey,
    pub params: LockerParams,
}

#[derive(Debug, BorshDeserialize)]
pub struct LockerParams {
    pub whitelist_enabled: bool,
    pub max_stake_vote_multiplier: u8,
    pub min_stake_duration: u64,
    pub max_stake_duration: u64,
    pub proposal_activation_min_votes: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct LockerWhitelistEntry {
    pub bump: u8,
    pub locker: Pubkey,
    pub program_id: Pubkey,
    pub owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct Proxy {
    pub escrow: Pubkey,
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    pub bump: u8,
    pub proxy_token_account: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct ProxyEscrow {
    pub escrow_owner: Pubkey,
    pub bump: u8,
    pub amount: u64,
    pub escrow_started_at: i64,
    pub escrow_ends_at: i64,
    pub rewards_last_claimed_at: i64,
    pub amount_claimed: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct RegisteredLocker {
    pub admin: Pubkey,
    pub bump: u8,
    pub token_mint: Pubkey,
    pub locker: Pubkey,
    pub padding: [u8; 7],
    pub reward_amount_paid_per_period_era_0: [u64; 256],
    pub reward_amount_paid_per_period_era_1: [u64; 256],
    pub reward_amount_paid_per_period_era_2: [u64; 256],
}

#[derive(Debug, BorshDeserialize)]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub register_stake: Pubkey,
    pub stake_mint: Pubkey,
    pub total_stake: u64,
    pub active_stake: u64,
    pub pending_rewards: u64,
    pub paid_rewards: u64,
    pub current_period: u16,
    pub staked_at_ts: i64,
    pub last_pending_reward_calc_ts: i64,
    pub last_harvest_ts: i64,
    pub unstaked_ts: i64,
    pub bump: u8,
}

// https://github.com/skullnbonesdao/dapp.skullnbones.xyz/blob/master/src/components/Accounts/AccountTemplate.vue
// const ATLAS_LOCKER_PROGRAM = 'ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc';
// const ATLAS_LOCKER_PROGRAM_AUTH =  'ATLkZsBofSKG845dNFpNoUyMciGpeH29BCbMqYFUoxzU';
