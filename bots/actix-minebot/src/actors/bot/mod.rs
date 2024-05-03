use super::*;

use std::sync::{Arc, Mutex};

mod autoplay;
pub use autoplay::BotOps;

mod tick;
pub use tick::*;

mod response;

pub struct BotActor {
    fleet: (Pubkey, Fleet),
    pub(crate) planet: (Pubkey, Planet),
    pub(crate) mine_item: (Pubkey, MineItem),
    pub(crate) resource: (Pubkey, Resource),

    // pub(crate) fleet: Option<Fleet>,
    pub(crate) fleet_state: Option<FleetState>,

    pub(crate) fleet_cargo_hold: Vec<(String, u64)>,
    pub(crate) fleet_fuel_tank: Vec<(String, u64)>,
    pub(crate) fleet_ammo_bank: Vec<(String, u64)>,
    pub(crate) fleet_food_cargo: Vec<(String, u64)>,

    clock: Option<Clock>,
    pub(crate) addr_sage: Addr<SageBasedActor>,
    pub(crate) operation: Option<autoplay::BotOps>,
    db: Arc<Mutex<db::MinebotDB>>,
}

impl BotActor {
    pub fn new(
        fleet: (Pubkey, Fleet),
        planet: (Pubkey, Planet),
        mine_item: (Pubkey, MineItem),
        resource: (Pubkey, Resource),
        addr_sage: Addr<SageBasedActor>,
        db: Arc<Mutex<db::MinebotDB>>,
    ) -> Self {
        Self {
            fleet,
            planet,
            mine_item,
            resource,
            fleet_state: None,
            fleet_cargo_hold: vec![],
            fleet_fuel_tank: vec![],
            fleet_ammo_bank: vec![],
            fleet_food_cargo: vec![],
            clock: None,
            addr_sage,
            operation: None,
            db,
        }
    }
}

impl Actor for BotActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping(pub Option<Signature>);

impl Handler<Ping> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) {
        log::info!("Pong: {:?}", msg.0);
        self.operation = None; // Clear operation
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClockTimeUpdate(pub Clock);

impl Handler<ClockTimeUpdate> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: ClockTimeUpdate, _: &mut Context<Self>) {
        let clock = msg.0;

        match &self.fleet_state {
            Some(FleetState::MineAsteroid(mine_asteroid)) => {
                let mining_ops = self.autoplay_mine_asteroid(&mine_asteroid, &clock);
                let operation = autoplay::BotOps::Mining(mining_ops);
                self.operation = Some(operation);
            }
            _ => {}
        }

        self.clock = Some(clock);
    }
}
