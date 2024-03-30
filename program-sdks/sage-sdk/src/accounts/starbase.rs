use super::*;

// TODO: finish Starbase Account struct
#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Starbase {
    pub version: u8,
    pub game_id: Pubkey,
    pub sector: [i64; 2],
    pub crafting_facility: Pubkey,
    pub name: [u8; 64],
    pub sub_coordinates: [i64; 2],
    pub faction: u8,
    pub bump: u8,
    pub seq_id: u16,
}

impl From<state::Starbase> for Starbase {
    fn from(s: state::Starbase) -> Self {
        Starbase {
            version: s.version,
            game_id: s.game_id,
            sector: s.sector,
            crafting_facility: s.crafting_facility,
            name: s.name,
            sub_coordinates: s.sub_coordinates,
            faction: s.faction,
            bump: s.bump,
            seq_id: s.seq_id,
        }
    }
}
