use super::*;

pub(crate) mod cargo_transport;
pub(crate) mod mine_asteroid;

pub enum BotRole {
    MineAsteroid {
        planet: (Pubkey, Planet),
        mine_item: (Pubkey, MineItem),
        resource: (Pubkey, Resource),
        // pub(crate) fleet_cargo_hold: Vec<(String, u64)>,
        // pub(crate) fleet_fuel_tank: Vec<(String, u64)>,
        // pub(crate) fleet_ammo_bank: Vec<(String, u64)>,
        // pub(crate) fleet_food_cargo: Vec<(String, u64)>,
    },
    CargoTransport {
        cargo_mint: Pubkey,
        cargo_amount: u64,
        from_sector: [i64; 2],
        from_starbase: Pubkey,
        to_sector: [i64; 2],
        to_starbase: Pubkey,
    },
}
