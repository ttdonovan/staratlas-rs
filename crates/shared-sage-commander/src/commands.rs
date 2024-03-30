use anchor_lang::prelude::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

use shared_sage_context::{Fleet, FleetState, Planet};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Command {
    Find(Find),
    Inquiry(Inquiry),
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Inquiry {
    Fleet(String),
    FleetState(String),
    FleetWithState(String),
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Find {
    Planets([i64; 2]),
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Reply {
    Fleet(Fleet),
    FleetState(FleetState),
    FleetWithState((Fleet, FleetState)),
    Planets(Vec<(Pubkey, Planet)>),
}

// #[derive(Debug, BorshSerialize, BorshDeserialize)]
// pub enum FleetCargoPods {
//     FuelTank,
//     AmmoBank,
//     CargoHold,
// }

// impl Command {
//     pub fn inquiry_fleet_cargo_hold_amount(fleet_id: String) {
//         // Command::Inquiry(Inquiry::FleetCargoPodsAmount(...)
//     }

//     pub fn inquiry_fleet_fuel_tank_amount() {

//     }
// }