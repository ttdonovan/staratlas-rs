use super::*;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct BaseEmissionsBySizeUtil {
    pub xx_small: u64,
    pub x_small: u64,
    pub small: u64,
    pub medium: u64,
    pub large: u64,
    pub capital: u64,
    pub commander: u64,
    pub titan: u64,
}

impl From<typedefs::BaseEmissionsBySizeUtil> for BaseEmissionsBySizeUtil {
    fn from(b: typedefs::BaseEmissionsBySizeUtil) -> Self {
        BaseEmissionsBySizeUtil {
            xx_small: b.xx_small,
            x_small: b.x_small,
            small: b.small,
            medium: b.medium,
            large: b.large,
            capital: b.capital,
            commander: b.commander,
            titan: b.titan,
        }
    }
}

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
pub struct FactionsStarbaseLevelInfo {
    // pub mud: [StarbaseLevelInfo; 7],
    // pub oni: [StarbaseLevelInfo; 7],
    // pub ustur: [StarbaseLevelInfo; 7],
}

impl From<typedefs::FactionsStarbaseLevelInfo> for FactionsStarbaseLevelInfo {
    fn from(_f: typedefs::FactionsStarbaseLevelInfo) -> Self {
        FactionsStarbaseLevelInfo {
            // mud: f.mud.into(),
            // oni: f.oni.into(),
            // ustur: f.ustur.into(),
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct FleetInfo {
    starbase_levels: FactionsStarbaseLevelInfo,
    fleets_lp_modifier: FleetsPointModifier,
    fleets_xp_modifier: FleetsPointModifier,
    max_fleet_size: u32,
}

impl From<typedefs::FleetInfo> for FleetInfo {
    fn from(f: typedefs::FleetInfo) -> Self {
        FleetInfo {
            starbase_levels: f.starbase_levels.into(),
            fleets_lp_modifier: f.fleets_lp_modifier.into(),
            fleets_xp_modifier: f.fleets_xp_modifier.into(),
            max_fleet_size: f.max_fleet_size,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct FleetsPointModifier {
    pub pubkey: Pubkey,
    pub bump: u8,
}

impl From<typedefs::FleetsPointModifier> for FleetsPointModifier {
    fn from(f: typedefs::FleetsPointModifier) -> Self {
        FleetsPointModifier {
            pubkey: f.pubkey,
            bump: f.bump,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Levers {
    pub l0_resources_scalar_multiplication: u64,
    pub l1_emissions_main_breaker: u64,
    pub l2_system_richness_emissions: u64,
    pub l3_ship_size_weight: u64,
    pub l4_resource_hardness: u64,
    pub l5_fuel_warp_breaker: u64,
    pub l6_fuel_planet_breaker: u64,
    pub l7_fuel_refinement_efficiency: u64,
    pub l8_mining_food_breaker: u64,
    pub l10_food_refinement_efficiency: u64,
    pub l11_organics_scalar_multiplication: u64,
    pub l16_fuel_combat_breaker: u64,
    pub l21_fuel_subwarp_breaker: u64,
    pub base_emissions_by_size_util: BaseEmissionsBySizeUtil,
}

impl From<typedefs::Levers> for Levers {
    fn from(l: typedefs::Levers) -> Self {
        Levers {
            l0_resources_scalar_multiplication: l.l0_resources_scalar_multiplication,
            l1_emissions_main_breaker: l.l1_emissions_main_breaker,
            l2_system_richness_emissions: l.l2_system_richness_emissions,
            l3_ship_size_weight: l.l3_ship_size_weight,
            l4_resource_hardness: l.l4_resource_hardness,
            l5_fuel_warp_breaker: l.l5_fuel_warp_breaker,
            l6_fuel_planet_breaker: l.l6_fuel_planet_breaker,
            l7_fuel_refinement_efficiency: l.l7_fuel_refinement_efficiency,
            l8_mining_food_breaker: l.l8_mining_food_breaker,
            l10_food_refinement_efficiency: l.l10_food_refinement_efficiency,
            l11_organics_scalar_multiplication: l.l11_organics_scalar_multiplication,
            l16_fuel_combat_breaker: l.l16_fuel_combat_breaker,
            l21_fuel_subwarp_breaker: l.l21_fuel_subwarp_breaker,
            base_emissions_by_size_util: l.base_emissions_by_size_util.into(),
        }
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
    pub scan_repair_kit_amount: u32,
}

impl From<typedefs::MiscStats> for MiscStats {
    fn from(m: typedefs::MiscStats) -> Self {
        MiscStats {
            crew: m.crew,
            respawn_time: m.respawn_time,
            scan_cool_down: m.scan_cool_down,
            scan_repair_kit_amount: m.scan_repair_kit_amount,
        }
    }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct MiscVariables {
    pub warp_lane_fuel_cost_reduction: i32,
    pub respawn_fee: u64,
}

impl From<typedefs::MiscVariables> for MiscVariables {
    fn from(m: typedefs::MiscVariables) -> Self {
        MiscVariables {
            warp_lane_fuel_cost_reduction: m.warp_lane_fuel_cost_reduction,
            respawn_fee: m.respawn_fee,
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

// #[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
// pub struct OptionalNonSystemPubkey {
//     pub key: Pubkey,
// }

// impl From<typedefs::OptionalNonSystemPubkey> for OptionalNonSystemPubkey {
//     fn from(o: typedefs::OptionalNonSystemPubkey) -> Self {
//         OptionalNonSystemPubkey {
//             key: o.key,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Points {
    pub xp_points_category: Pubkey,
    pub lp_points_category: Pubkey,
}

impl From<typedefs::Points> for Points {
    fn from(p: typedefs::Points) -> Self {
        Points {
            xp_points_category: p.xp_points_category,
            lp_points_category: p.lp_points_category,
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
pub struct StarbaseLevelInfo {
    recipe_for_upgrade: Pubkey,
    recipe_category_for_level: Pubkey,
    hp: u64,
    sp: u64,
    sector_ring_available: u8,
    warp_lane_movement_fee: u64,
}

impl From<typedefs::StarbaseLevelInfo> for StarbaseLevelInfo {
    fn from(s: typedefs::StarbaseLevelInfo) -> Self {
        StarbaseLevelInfo {
            recipe_for_upgrade: s.recipe_for_upgrade,
            recipe_category_for_level: s.recipe_category_for_level,
            hp: s.hp,
            sp: s.sp,
            sector_ring_available: s.sector_ring_available,
            warp_lane_movement_fee: s.warp_lane_movement_fee,
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
