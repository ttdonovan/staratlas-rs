use anchor_client::solana_sdk::pubkey::Pubkey;

use staratlas_sage_sdk::accounts::{Fleet, FleetState};

pub trait FleetWithState {
    fn fleet_id(&self) -> &Pubkey;
    fn fleet_acct(&self) -> &Fleet;
    fn fleet_state(&self) -> &FleetState;
}
