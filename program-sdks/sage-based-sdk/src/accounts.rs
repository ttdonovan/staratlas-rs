use anchor_client::anchor_lang::{prelude::Pubkey, AnchorDeserialize};
use borsh::{BorshDeserialize, BorshSerialize};

use std::io::Read;

pub use staratlas_sage::{state, typedefs};

mod cargo;
pub use cargo::*;

mod fleet;
pub use fleet::*;

mod fleet_state;
pub use fleet_state::*;

mod game;
pub use game::*;

mod mine_item;
pub use mine_item::*;

mod planet;
pub use planet::*;

mod resource;
pub use resource::*;

pub mod types;

#[derive(Debug, Clone)]
pub struct FleetWithState(pub Fleet, pub FleetState);

impl borsh::de::BorshDeserialize for FleetWithState {
    fn deserialize_reader<R: Read>(reader: &mut R) -> borsh::io::Result<Self> {
        let mut account_data = vec![];
        reader.read_to_end(&mut account_data)?;

        let account_data = account_data.as_slice();
        let mut account_data = &account_data[8..];

        let fleet = Fleet::deserialize_reader(&mut account_data)?;

        let discriminator = account_data[0];
        let mut remaining_data = &account_data[1..];

        let state = match discriminator {
            0 => {
                let starbase_loading_bay =
                    typedefs::StarbaseLoadingBay::deserialize(&mut remaining_data)?;
                FleetState::StarbaseLoadingBay(starbase_loading_bay.into())
            }
            1 => {
                let idle = typedefs::Idle::deserialize(&mut remaining_data)?;
                FleetState::Idle(idle.into())
            }
            2 => {
                let mine_astriod = typedefs::MineAsteroid::deserialize(&mut remaining_data)?;
                FleetState::MineAsteroid(mine_astriod.into())
            }
            3 => {
                let move_warp = typedefs::MoveWarp::deserialize(&mut remaining_data)?;
                FleetState::MoveWarp(move_warp.into())
            }
            4 => {
                let move_subwarp = typedefs::MoveSubwarp::deserialize(&mut remaining_data)?;
                FleetState::MoveSubwarp(move_subwarp.into())
            }
            5 => {
                let respawn = typedefs::Respawn::deserialize(&mut remaining_data)?;
                FleetState::Respawn(respawn.into())
            }
            _ => {
                unreachable!("Fleet account has invalid FleetState discriminator")
            }
        };

        Ok(FleetWithState(fleet, state))
    }
}
