use super::*;

pub(crate) fn calc_asteroid_mining_emission_rate(
    fleet: &sage::Fleet,
    resource: &sage::Resource,
    mine_item: &sage::MineItem,
) -> f32 {
    let mining_rate = fleet.stats.cargo_stats.mining_rate as f32;
    let system_richness = resource.system_richness as f32;
    let resource_harndess = mine_item.resource_hardness as f32;

    (mining_rate / 10000.0) * (system_richness / resource_harndess)
}

pub(crate) fn calc_asteroid_mining_amount(
    cargo_amount: u32,
    cargo_capacity: u32,
    emission_rate: f32,
    mine_asteroid_start: i64,
) -> u32 {
    let mining_time_elapsed = if mine_asteroid_start == 0 {
        0
    } else {
        time::get_time() as i64 - mine_asteroid_start
    };
    let est_amount_minded = mining_time_elapsed as f32 * emission_rate;
    let mut est_cargo_amount = cargo_amount + est_amount_minded as u32;
    est_cargo_amount = est_cargo_amount.min(cargo_capacity);
    cargo_capacity - est_cargo_amount
}

pub(crate) fn calc_asteroid_mining_duration(amount: u32, emission_rate: f32) -> Duration {
    Duration::from_secs_f32(amount as f32 / emission_rate)
}
