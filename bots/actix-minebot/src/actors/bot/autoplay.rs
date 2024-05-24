use super::*;

use crate::timers;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BotOps {
    Idle(IdleOps),
    Mining(MiningOps),
    StarbaseLoadingBay(StarbaseLoadingBayOps),
    TxsSageBased(TxsSageBasedOps),
    Warp(WarpOps),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) enum IdleActions {
    DockeToStarbase,
    MineAsteroid,
    WarpToSector([i64; 2]),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct IdleOps {
    pub(crate) sector: [i64; 2],
    pub(crate) cargo_capacity_fraction: f64,
    pub(crate) stopwatch: timers::Stopwatch,
    pub(crate) next_action: IdleActions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct MiningOps {
    pub(crate) mining_location: String,
    pub(crate) currently_mining: String,
    pub(crate) resource_mining_rate_per_second: f32,
    pub(crate) amount_mined: f32,
    pub(crate) timer: timers::Timer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct WarpOps {
    pub(crate) sector: [i64; 2],
    pub(crate) timer: timers::Timer,
    pub(crate) cooldown: timers::Timer,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct TxsSageBasedOps {
    pub(crate) stopwatch: timers::Stopwatch,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) enum StarbaseActions {
    IdleHangar,
    CargoDeposit(Pubkey, Pubkey, u64), // (CargoPodTo, Mint, Amount)
    CargoWithdraw(Pubkey, u64),        // (Mint, Amount)
    CheckFuelStatus,
    CheckAmmoStatus,
    CheckFoodStatus,
    UndockFromStarbase,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct StarbaseLoadingBayOps {
    pub(crate) starbase: Pubkey,
    pub(crate) stopwatch: timers::Stopwatch,
    pub(crate) next_action: StarbaseActions,
}
