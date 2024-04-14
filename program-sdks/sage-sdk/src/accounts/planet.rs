use super::*;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct MineItem {
    pub version: u8,
    pub game_id: Pubkey,
    pub name: [u8; 64],
    pub mint: Pubkey,
    pub resource_hardness: u16,
    pub num_resource_accounts: u64,
    pub bump: u8,
}

impl MineItem {
    pub fn name(&self) -> &str {
        let name = std::str::from_utf8(&self.name).unwrap();
        let name_trimmed = name.trim_end_matches(char::from(0));
        name_trimmed
    }
}

impl From<state::MineItem> for MineItem {
    fn from(m: state::MineItem) -> Self {
        MineItem {
            version: m.version,
            game_id: m.game_id,
            name: m.name,
            mint: m.mint,
            resource_hardness: m.resource_hardness,
            num_resource_accounts: m.num_resource_accounts,
            bump: m.bump,
        }
    }
}

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

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Resource {
    pub version: u8,
    pub game_id: Pubkey,
    pub location: Pubkey,
    pub mine_item: Pubkey,
    pub location_type: u8,
    pub system_richness: u16,
    pub amount_mined: u64,
    pub num_miners: u64,
    pub bump: u8,
}

impl From<state::Resource> for Resource {
    fn from(r: state::Resource) -> Self {
        Resource {
            version: r.version,
            game_id: r.game_id,
            location: r.location,
            mine_item: r.mine_item,
            location_type: r.location_type,
            system_richness: r.system_richness,
            amount_mined: r.amount_mined,
            num_miners: r.num_miners,
            bump: r.bump,
        }
    }
}
