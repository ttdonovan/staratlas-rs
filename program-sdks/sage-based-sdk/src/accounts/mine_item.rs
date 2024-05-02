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
