use super::*;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Fleet {
    pub version: u8,
    pub game_id: Pubkey,
    pub owner_profile: Pubkey,
    pub fleet_ships: Pubkey,
    pub sub_profile: SubProfile,
    pub sub_profile_invalidator: Pubkey,
    pub faction: u8,
    pub fleet_label: [u8; 32],
    pub ship_counts: types::ShipCounts,
    pub warp_cooldown_expires_at: i64,
    pub scan_cooldown_expires_at: i64,
    pub stats: types::ShipStats,
    pub cargo_hold: Pubkey,
    pub fuel_tank: Pubkey,
    pub ammo_bank: Pubkey,
    pub update_id: u64,
    pub bump: u8,
}

impl Fleet {
    pub fn name(&self) -> &str {
        let name = std::str::from_utf8(&self.fleet_label).unwrap();
        let name_trimmed = name.trim_end_matches(char::from(0));
        name_trimmed
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct SubProfile {
    pub key: Pubkey,
}

impl From<state::Fleet> for Fleet {
    fn from(f: state::Fleet) -> Self {
        Fleet {
            version: f.version,
            game_id: f.game_id,
            owner_profile: f.owner_profile,
            fleet_ships: f.fleet_ships,
            sub_profile: SubProfile {
                key: f.sub_profile.key,
            },
            sub_profile_invalidator: f.sub_profile_invalidator,
            faction: f.faction,
            fleet_label: f.fleet_label,
            ship_counts: f.ship_counts.into(),
            warp_cooldown_expires_at: f.warp_cooldown_expires_at,
            scan_cooldown_expires_at: f.scan_cooldown_expires_at,
            stats: f.stats.into(),
            cargo_hold: f.cargo_hold,
            fuel_tank: f.fuel_tank,
            ammo_bank: f.ammo_bank,
            update_id: f.update_id,
            bump: f.bump,
        }
    }
}
