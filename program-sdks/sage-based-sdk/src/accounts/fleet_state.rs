use super::*;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum FleetState {
    StarbaseLoadingBay(StarbaseLoadingBay),
    Idle(Idle),
    MineAsteroid(MineAsteroid),
    MoveWarp(MoveWarp),
    MoveSubwarp(MoveSubwarp),
    Respawn(Respawn),
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct StarbaseLoadingBay {
    pub starbase: Pubkey,
    pub last_update: i64,
}

impl From<typedefs::StarbaseLoadingBay> for StarbaseLoadingBay {
    fn from(s: typedefs::StarbaseLoadingBay) -> Self {
        StarbaseLoadingBay {
            starbase: s.starbase,
            last_update: s.last_update,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Idle {
    pub sector: [i64; 2],
}

impl From<typedefs::Idle> for Idle {
    fn from(s: typedefs::Idle) -> Self {
        Idle { sector: s.sector }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MineAsteroid {
    pub asteroid: Pubkey,
    pub resource: Pubkey,
    pub start: i64,
    pub end: i64,
    pub last_update: i64,
}

impl From<typedefs::MineAsteroid> for MineAsteroid {
    fn from(s: typedefs::MineAsteroid) -> Self {
        MineAsteroid {
            asteroid: s.asteroid,
            resource: s.resource,
            start: s.start,
            end: s.end,
            last_update: s.last_update,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MoveWarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub warp_start: i64,
    pub warp_finish: i64,
}

impl From<typedefs::MoveWarp> for MoveWarp {
    fn from(s: typedefs::MoveWarp) -> Self {
        MoveWarp {
            from_sector: s.from_sector,
            to_sector: s.to_sector,
            warp_start: s.warp_start,
            warp_finish: s.warp_finish,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MoveSubwarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub current_sector: [i64; 2],
    pub departure_time: i64,
    pub arrival_time: i64,
    pub fuel_expenditure: u64,
    pub last_update: i64,
}

impl From<typedefs::MoveSubwarp> for MoveSubwarp {
    fn from(s: typedefs::MoveSubwarp) -> Self {
        MoveSubwarp {
            from_sector: s.from_sector,
            to_sector: s.to_sector,
            current_sector: s.current_sector,
            departure_time: s.departure_time,
            arrival_time: s.arrival_time,
            fuel_expenditure: s.fuel_expenditure,
            last_update: s.last_update,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Respawn {
    pub sector: [i64; 2],
    pub start: i64,
}

impl From<typedefs::Respawn> for Respawn {
    fn from(s: typedefs::Respawn) -> Self {
        Respawn {
            sector: s.sector,
            start: s.start,
        }
    }
}
