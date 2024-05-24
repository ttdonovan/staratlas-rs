use super::*;

use crate::programs::staratlas_cargo::state;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct CargoPod {
    pub version: u8,
    pub stats_definition: Pubkey,
    pub open_token_accounts: u8,
    pub pod_seeds: [u8; 32],
    pub pod_bump: u8,
    pub seq_id: u16,
    pub unupdated_token_accounts: u8,
}

impl From<state::CargoPod> for CargoPod {
    fn from(c: state::CargoPod) -> Self {
        CargoPod {
            version: c.version,
            stats_definition: c.stats_definition,
            open_token_accounts: c.open_token_accounts,
            pod_seeds: c.pod_seeds,
            pod_bump: c.pod_bump,
            seq_id: c.seq_id,
            unupdated_token_accounts: c.unupdated_token_accounts,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct CargoStatsDefinition {
    pub version: u8,
    pub authority: Pubkey,
    pub default_cargo_type: Pubkey,
    pub stats_count: u16,
    pub seq_id: u16,
}

impl From<state::CargoStatsDefinition> for CargoStatsDefinition {
    fn from(c: state::CargoStatsDefinition) -> Self {
        CargoStatsDefinition {
            version: c.version,
            authority: c.authority,
            default_cargo_type: c.default_cargo_type,
            stats_count: c.stats_count,
            seq_id: c.seq_id,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct CargoType {
    pub version: u8,
    pub stats_definition: Pubkey,
    pub mint: Pubkey,
    pub stats_count: u16,
    pub seq_id: u16,
}

impl From<state::CargoType> for CargoType {
    fn from(c: state::CargoType) -> Self {
        CargoType {
            version: c.version,
            stats_definition: c.stats_definition,
            mint: c.mint,
            stats_count: c.stats_count,
            seq_id: c.seq_id,
        }
    }
}
