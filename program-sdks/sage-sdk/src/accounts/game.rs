use super::*;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Game {
    pub version: u8,
    pub update_id: u64,
    pub profile: Pubkey,
    pub game_state: Pubkey,
    pub points: types::Points,
    pub cargo: types::Cargo,
    pub crafting: types::Crafting,
    pub mints: types::Mints,
    pub vaults: types::Vaults,
    pub risk_zones: types::RiskZonesData,
}

impl From<state::Game> for Game {
    fn from(g: state::Game) -> Self {
        Game {
            version: g.version,
            update_id: g.update_id,
            profile: g.profile,
            game_state: g.game_state,
            points: g.points.into(),
            cargo: g.cargo.into(),
            crafting: g.crafting.into(),
            mints: g.mints.into(),
            vaults: g.vaults.into(),
            risk_zones: g.risk_zones.into(),
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct GameState {
    pub version: u8,
    pub update_id: u64,
    pub game_id: Pubkey,
    pub fleet: types::FleetInfo,
    pub levers: types::Levers,
    pub misc: types::MiscVariables,
    pub bump: u8,
}

impl From<state::GameState> for GameState {
    fn from(g: state::GameState) -> Self {
        GameState {
            version: g.version,
            update_id: g.update_id,
            game_id: g.game_id,
            fleet: g.fleet.into(),
            levers: g.levers.into(),
            misc: g.misc.into(),
            bump: g.bump,
        }
    }
}
