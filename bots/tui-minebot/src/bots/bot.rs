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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CargoDeposit {
    Fuel,
    Ammo,
    Food,
}

#[derive(Debug)]
pub struct MiningBot {
    pub fleet_id: Pubkey,
    pub masked_fleet_id: String,
    pub fleet_name: String,
    pub fleet: sage::Fleet,
    pub fleet_state: sage::FleetState,
    pub fuel_tank: Pubkey,
    pub fuel_tank_amount: u32,
    pub fuel_tank_capacity: u32,
    pub ammo_bank: Pubkey,
    pub ammo_bank_amount: u32,
    pub ammo_bank_capacity: u32,
    pub cargo_hold: Pubkey,
    pub cargo_hold_amount: u32,
    pub cargo_hold_capacity: u32,
    pub resource: (Pubkey, sage::Resource),
    pub mine_item: (Pubkey, sage::MineItem),
    pub mine_item_name: String,
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
    pub fn fleet(&self) -> (Pubkey, sage::Fleet, sage::FleetState) {
        (self.fleet_id, self.fleet, self.fleet_state.clone())
    }

    pub fn set_fleet_state(&mut self, fleet_state: sage::FleetState) {
        self.fleet_state = fleet_state;
    }

    pub fn masked_fleet_id(&self) -> &String {
        &self.masked_fleet_id
    }

    pub fn fleet_name(&self) -> &String {
        &self.fleet_name
    }

    pub fn mine_rate(&self) -> f32 {
        self.mine_asteroid_emission_rate
    }

    pub fn mine_amount(&self) -> u32 {
        self.mine_asteroid_amount
    }

    pub fn mine_start(&self) -> i64 {
        match &self.fleet_state {
            sage::FleetState::MineAsteroid(mine_asteroid) => mine_asteroid.start,
            _ => 0,
        }
    }

    pub fn mine_duration(&self) -> Duration {
        self.mine_asteroid_duraiton
    }

    pub fn starbase_id(&self) -> Option<&Pubkey> {
        match &self.fleet_state {
            sage::FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
                Some(&starbase_loading_bay.starbase)
            }
            _ => None,
        }
    }

    pub fn is_fleet_idle(&self) -> bool {
        match &self.fleet_state {
            sage::FleetState::Idle(_) => true,
            _ => false,
        }
    }

    pub fn is_fleet_mining(&self) -> bool {
        match &self.fleet_state {
            sage::FleetState::MineAsteroid(_) => true,
            _ => false,
        }
    }

    pub fn is_fleet_at_starbase(&self) -> bool {
        match &self.fleet_state {
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
        let amount = amount.min(self.fuel_tank_capacity);
        self.fuel_tank_amount = amount;
    }

    pub fn set_ammo_amount(&mut self, amount: u32) {
        let amount = amount.min(self.ammo_bank_capacity);
        self.ammo_bank_amount = amount;
    }

    pub fn set_cargo_amount(&mut self, amount: u32) {
        let amount = amount.min(self.cargo_hold_capacity);
        self.cargo_hold_amount = amount;
    }

    pub fn tick(&mut self, dt: Duration) {
        self.mining_timer.tick(dt);
    }

    pub fn reset_mining_timer(&mut self) {
        self.mine_asteroid_amount = calc_asteroid_mining_amount(
            self.cargo_hold_amount,
            self.cargo_hold_capacity,
            self.mine_asteroid_emission_rate,
            self.mine_start(),
        );

        self.mine_asteroid_duraiton = calc_asteroid_mining_duration(
            self.mine_asteroid_amount,
            self.mine_asteroid_emission_rate,
        );

        self.mining_timer.set_duration(self.mine_asteroid_duraiton);
        self.mining_timer.reset();
    }
}
