use crate::accounts::types;
use crate::accounts::*;

const GLOBAL_SCALE_DECIMALS_4: f32 = 10_000.0;

const RESOURCE_HARDNESS_DECIMALS: f32 = 100.0;
const SYSTEM_RICHNESS_DECIMALS: f32 = 100.0;

pub fn asteroid_mining_resource_extraction_duration(
    fleet_stats: &types::ShipStats,
    mine_item: &MineItem,
    resource: &Resource,
    resource_amount: u32,
) -> f32 {
    asteroid_mining_resource_extraction_duration_bare_bones(
        fleet_stats,
        mine_item.resource_hardness,
        resource.system_richness,
        resource_amount,
    )
}

pub fn asteroid_mining_resource_extraction_duration_bare_bones(
    fleet_stats: &types::ShipStats,
    resource_hardness: u16,
    system_richness: u16,
    resource_amount: u32,
) -> f32 {
    let emission_rate =
        asteroid_mining_emssion_rate_bare_bones(fleet_stats, resource_hardness, system_richness);
    if emission_rate > 0.0 {
        return resource_amount as f32 / emission_rate;
    }

    return 0.0;
}

pub fn asteroid_mining_emission_rate(
    fleet_stats: &types::ShipStats,
    mine_item: &MineItem,
    resource: &Resource,
) -> f32 {
    asteroid_mining_emssion_rate_bare_bones(
        fleet_stats,
        mine_item.resource_hardness,
        resource.system_richness,
    )
}

pub fn asteroid_mining_emssion_rate_bare_bones(
    fleet_stats: &types::ShipStats,
    resource_hardness: u16,
    system_richness: u16,
) -> f32 {
    let resource_hardness = resource_hardness as f32 / RESOURCE_HARDNESS_DECIMALS;
    let system_richness = system_richness as f32 / SYSTEM_RICHNESS_DECIMALS;
    let base_rate = (fleet_stats.cargo_stats.mining_rate as f32 / GLOBAL_SCALE_DECIMALS_4)
        * system_richness
        / resource_hardness;
    base_rate
}
