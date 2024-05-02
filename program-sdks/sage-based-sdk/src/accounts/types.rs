use super::*;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Cargo {
    pub stats_definition: Pubkey,
}

impl From<typedefs::Cargo> for Cargo {
    fn from(c: typedefs::Cargo) -> Self {
        Cargo {
            stats_definition: c.stats_definition,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct CargoStats {
    pub cargo_capacity: u32,
    pub fuel_capacity: u32,
    pub ammo_capacity: u32,
    pub ammo_consumption_rate: u32,
    pub food_consumption_rate: u32,
    pub mining_rate: u32,
    pub upgrade_rate: u32,
    pub cargo_transfer_rate: u32,
    pub tractor_beam_gather_rate: u32,
}

impl From<typedefs::CargoStats> for CargoStats {
    fn from(c: typedefs::CargoStats) -> Self {
        CargoStats {
            cargo_capacity: c.cargo_capacity,
            fuel_capacity: c.fuel_capacity,
            ammo_capacity: c.ammo_capacity,
            ammo_consumption_rate: c.ammo_consumption_rate,
            food_consumption_rate: c.food_consumption_rate,
            mining_rate: c.mining_rate,
            upgrade_rate: c.upgrade_rate,
            cargo_transfer_rate: c.cargo_transfer_rate,
            tractor_beam_gather_rate: c.tractor_beam_gather_rate,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Crafting {
    pub domain: Pubkey,
}

impl From<typedefs::Crafting> for Crafting {
    fn from(c: typedefs::Crafting) -> Self {
        Crafting { domain: c.domain }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Mints {
    pub atlas: Pubkey,
    pub polis: Pubkey,
    pub ammo: Pubkey,
    pub food: Pubkey,
    pub fuel: Pubkey,
    pub repair_kit: Pubkey,
}

impl From<typedefs::Mints> for Mints {
    fn from(m: typedefs::Mints) -> Self {
        Mints {
            atlas: m.atlas,
            polis: m.polis,
            ammo: m.ammo,
            food: m.food,
            fuel: m.fuel,
            repair_kit: m.repair_kit,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct MiscStats {
    pub crew: u64,
    pub respawn_time: u16,
    pub scan_cool_down: u16,
    pub sdu_per_scan: u32,
    pub scan_cost: u32,
    pub placeholder: u32,
    pub placeholder2: u32,
    pub placeholder3: u32,
}

impl From<typedefs::MiscStats> for MiscStats {
    fn from(m: typedefs::MiscStats) -> Self {
        MiscStats {
            crew: m.crew,
            respawn_time: m.respawn_time,
            scan_cool_down: m.scan_cool_down,
            sdu_per_scan: m.sdu_per_scan,
            scan_cost: m.scan_cost,
            placeholder: m.placeholder,
            placeholder2: m.placeholder2,
            placeholder3: m.placeholder3,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct MovementStats {
    pub subwarp_speed: u32,
    pub warp_speed: u32,
    pub max_warp_distance: u16,
    pub warp_cool_down: u16,
    pub subwarp_fuel_consumption_rate: u32,
    pub warp_fuel_consumption_rate: u32,
    pub planet_exit_fuel_amount: u32,
}

impl From<typedefs::MovementStats> for MovementStats {
    fn from(m: typedefs::MovementStats) -> Self {
        MovementStats {
            subwarp_speed: m.subwarp_speed,
            warp_speed: m.warp_speed,
            max_warp_distance: m.max_warp_distance,
            warp_cool_down: m.warp_cool_down,
            subwarp_fuel_consumption_rate: m.subwarp_fuel_consumption_rate,
            warp_fuel_consumption_rate: m.warp_fuel_consumption_rate,
            planet_exit_fuel_amount: m.planet_exit_fuel_amount,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Points {
    pub lp_category: SagePointsCategory,
    pub council_rank_xp_category: SagePointsCategory,
    pub pilot_xp_category: SagePointsCategory,
    pub data_running_xp_category: SagePointsCategory,
    pub mining_xp_category: SagePointsCategory,
    pub crafting_xp_category: SagePointsCategory,
}

impl From<typedefs::Points> for Points {
    fn from(p: typedefs::Points) -> Self {
        Points {
            lp_category: p.lp_category.into(),
            council_rank_xp_category: p.council_rank_xp_category.into(),
            pilot_xp_category: p.pilot_xp_category.into(),
            data_running_xp_category: p.data_running_xp_category.into(),
            mining_xp_category: p.mining_xp_category.into(),
            crafting_xp_category: p.crafting_xp_category.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct RiskZoneData {
    pub center: [i64; 2],
    pub radius: u64,
}

impl From<typedefs::RiskZoneData> for RiskZoneData {
    fn from(r: typedefs::RiskZoneData) -> Self {
        RiskZoneData {
            center: r.center,
            radius: r.radius,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct RiskZonesData {
    pub mud_security_zone: RiskZoneData,
    pub oni_security_zone: RiskZoneData,
    pub ustur_security_zone: RiskZoneData,
    pub high_risk_zone: RiskZoneData,
    pub medium_risk_zone: RiskZoneData,
}

impl From<typedefs::RiskZonesData> for RiskZonesData {
    fn from(r: typedefs::RiskZonesData) -> Self {
        RiskZonesData {
            mud_security_zone: r.mud_security_zone.into(),
            oni_security_zone: r.oni_security_zone.into(),
            ustur_security_zone: r.ustur_security_zone.into(),
            high_risk_zone: r.high_risk_zone.into(),
            medium_risk_zone: r.medium_risk_zone.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct SagePointsCategory {
    pub category: Pubkey,
    pub modifier: Pubkey,
    pub modifier_bump: u8,
}

impl From<typedefs::SagePointsCategory> for SagePointsCategory {
    fn from(s: typedefs::SagePointsCategory) -> Self {
        SagePointsCategory {
            category: s.category,
            modifier: s.modifier,
            modifier_bump: s.modifier_bump,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct ShipCounts {
    pub total: u32,
    pub updated: u32,
    pub xx_small: u16,
    pub x_small: u16,
    pub small: u16,
    pub medium: u16,
    pub large: u16,
    pub capital: u16,
    pub commander: u16,
    pub titan: u16,
}

impl From<typedefs::ShipCounts> for ShipCounts {
    fn from(s: typedefs::ShipCounts) -> Self {
        ShipCounts {
            total: s.total,
            updated: s.updated,
            xx_small: s.xx_small,
            x_small: s.x_small,
            small: s.small,
            medium: s.medium,
            large: s.large,
            capital: s.capital,
            commander: s.commander,
            titan: s.titan,
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct ShipStats {
    pub movement_stats: MovementStats,
    pub cargo_stats: CargoStats,
    pub misc_stats: MiscStats,
}

impl From<typedefs::ShipStats> for ShipStats {
    fn from(s: typedefs::ShipStats) -> Self {
        ShipStats {
            movement_stats: s.movement_stats.into(),
            cargo_stats: s.cargo_stats.into(),
            misc_stats: s.misc_stats.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Vaults {
    pub atlas: Pubkey,
    pub polis: Pubkey,
}

impl From<typedefs::Vaults> for Vaults {
    fn from(v: typedefs::Vaults) -> Self {
        Vaults {
            atlas: v.atlas,
            polis: v.polis,
        }
    }
}
