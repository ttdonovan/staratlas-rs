use super::*;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Planet {
    pub version: u8,
    pub name: [u8; 64],
    pub game_id: Pubkey,
    pub sector: [i64; 2],
    pub sub_coordinates: [i64; 2],
    pub planet_type: u8,
    pub position: u8,
    pub size: u64,
    pub max_hp: u64,
    pub current_health: u64,
    pub amount_mined: u64,
    pub num_resources: u8,
    pub num_miners: u64,
}

impl Planet {
    pub fn name(&self) -> &str {
        let name = std::str::from_utf8(&self.name).unwrap();
        let name_trimmed = name.trim_end_matches(char::from(0));
        name_trimmed
    }
}

impl From<state::Planet> for Planet {
    fn from(p: state::Planet) -> Self {
        Planet {
            version: p.version,
            name: p.name,
            game_id: p.game_id,
            sector: p.sector,
            sub_coordinates: p.sub_coordinates,
            planet_type: p.planet_type,
            position: p.position,
            size: p.size,
            max_hp: p.max_hp,
            current_health: p.current_health,
            amount_mined: p.amount_mined,
            num_resources: p.num_resources,
            num_miners: p.num_miners,
        }
    }
}
