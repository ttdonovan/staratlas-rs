use super::*;

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
