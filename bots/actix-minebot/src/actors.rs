use actix::prelude::*;
use anchor_client::{
    anchor_lang::prelude::{Clock, Pubkey},
    solana_sdk::signature::{Keypair, Signature},
    Client,
};
use serde::{Deserialize, Serialize};

use staratlas_sage_based_sdk::{
    calc,
    program::{CARGO_ID, SAGE_ID},
    Fleet, FleetState, FleetWithState, Game, Idle, MineAsteroid, MineItem, Planet, Resource,
    SageBasedGameHandler, StarbaseLoadingBay,
};

use std::rc::Rc;
use std::str::FromStr;

use crate::{db, timers};

mod bot;
pub use bot::*;

mod sage;
pub use sage::*;
