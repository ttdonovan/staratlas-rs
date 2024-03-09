use super::*;

#[derive(Debug, PartialEq)]
pub enum Autoplay {
    IsIdle,
    StartMiningAsteroid,
    IsMiningAsteroid,
    StopMiningAsteroid,
    StarbaseDock,
    StarbaseUndock,
    StarbaseHangarCargoWithdraw,
    StarbaseHangarCargoDeposit(CargoDeposit),
}

#[derive(Debug, PartialEq)]
pub enum CargoDeposit {
    Fuel,
    Ammo,
    Food,
}

pub type Fleet = (Pubkey, String, String, sage::Fleet, sage::FleetState); // (Pubkey, mask, callsign, fleet, fleet_state)
pub type FuelTank = (Pubkey, u32, u32); // (Pubkey, amount, capacity)
pub type AmmoBank = (Pubkey, u32, u32); // (Pubkey, amount, capacity)
pub type CargoHold = (Pubkey, u32, u32); // (Pubkey, amount, capacity)
pub type Resource = (Pubkey, sage::Resource);
pub type MineItem = (Pubkey, String, sage::MineItem);

#[derive(Debug)]
pub struct MiningBot {
    pub fleet: Fleet,
    pub is_fleet_state_dirty: bool,
    pub fuel_tank: FuelTank,
    pub ammo_bank: AmmoBank,
    pub cargo_hold: CargoHold,
    pub resource: Resource,
    pub mine_item: MineItem,
    pub mine_asteroid_emission_rate: f32,
    pub mine_asteroid_amount: u32,
    pub mine_asteroid_duraiton: Duration,
    pub mining_timer: time::Timer,
    pub autoplay: Autoplay,
    pub txs: Option<Signature>,
    pub txs_counter: u32,
    pub txs_errors: u32,
    pub is_tx: bool,
}

impl MiningBot {
    pub fn masked_fleet_id(&self) -> &str {
        &self.fleet.1
    }

    pub fn fleet_name(&self) -> &str {
        &self.fleet.2
    }

    pub fn mine_item_name(&self) -> &str {
        &self.mine_item.1
    }

    pub fn mine_rate(&self) -> f32 {
        self.mine_asteroid_emission_rate
    }

    pub fn mine_amount(&self) -> u32 {
        self.mine_asteroid_amount
    }

    pub fn mine_start(&self) -> i64 {
        match &self.fleet.4 {
            FleetState::MineAsteroid(mine_asteroid) => mine_asteroid.start,
            _ => 0,
        }
    }

    pub fn mine_duration(&self) -> Duration {
        self.mine_asteroid_duraiton
    }

    pub fn starbase_id(&self) -> Option<&Pubkey> {
        match &self.fleet.4 {
            sage::FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                Some(&starbase_loading_bay.starbase)
            }
            _ => None,
        }
    }

    pub fn is_fleet_idle(&self) -> bool {
        match &self.fleet.4 {
            sage::FleetState::Idle(_) => true,
            _ => false,
        }
    }

    pub fn is_fleet_mining(&self) -> bool {
        match &self.fleet.4 {
            sage::FleetState::MineAsteroid(_) => true,
            _ => false,
        }
    }

    pub fn is_fleet_at_starbase(&self) -> bool {
        match &self.fleet.4 {
            sage::FleetState::StarbaseLoadingBay(_) => true,
            _ => false,
        }
    }

    pub fn last_txs(&self) -> String {
        match self.txs.as_ref() {
            Some(signature) => signature.to_string(),
            None => String::from(""),
        }
    }

    pub fn set_fuel_amount(&mut self, amount: u32) {
        let amount = amount.min(self.fuel_tank.2);
        self.fuel_tank.1 = amount;
    }

    pub fn set_ammo_amount(&mut self, amount: u32) {
        let amount = amount.min(self.ammo_bank.2);
        self.ammo_bank.1 = amount;
    }

    pub fn set_cargo_amount(&mut self, amount: u32) {
        let amount = amount.min(self.cargo_hold.2);
        self.cargo_hold.1 = amount;
    }

    pub fn tick(&mut self, dt: Duration) {
        self.mining_timer.tick(dt);
    }

    pub fn reset_mining_timer(&mut self) {
        self.mining_timer.set_duration(self.mine_asteroid_duraiton);
        self.mining_timer.reset();
    }
}
