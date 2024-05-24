use super::*;

use std::sync::{Arc, Mutex};

mod autoplay;
pub use autoplay::*;

mod roles;
pub use roles::*;

pub struct BotActor {
    db: Arc<Mutex<db::MinebotDB>>,
    clock: Option<Clock>,
    pub(crate) operation: Option<BotOps>,
    pub(crate) addr_sage: Addr<SageBasedActor>,
    role: BotRole,
    fleet: (Pubkey, FleetWithState),
    pub(crate) fleet_cargo_hold: Vec<(String, u64)>,
    pub(crate) fleet_fuel_tank: Vec<(String, u64)>,
    pub(crate) fleet_ammo_bank: Vec<(String, u64)>,
    pub(crate) fleet_food_cargo: Vec<(String, u64)>,
}

impl BotActor {
    pub fn new(
        db: Arc<Mutex<db::MinebotDB>>,
        addr_sage: Addr<SageBasedActor>,
        fleet: (Pubkey, FleetWithState),
        role: BotRole,
        // planet: (Pubkey, Planet),
        // mine_item: (Pubkey, MineItem),
        // resource: (Pubkey, Resource),
    ) -> Self {
        Self {
            db,
            addr_sage,
            fleet,
            role,
            // role: BotRole::MineAsteroid {
            //     planet,
            //     mine_item,
            //     resource,
            // },
            fleet_cargo_hold: vec![],
            fleet_fuel_tank: vec![],
            fleet_ammo_bank: vec![],
            fleet_food_cargo: vec![],
            clock: None,
            operation: None,
        }
    }
}

impl BotActor {
    pub fn fleet_state(&self) -> &FleetState {
        let (_, FleetWithState(_, state)) = &self.fleet;
        state
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
        match &self.role {
            BotRole::MineAsteroid { .. } => {
                roles::mine_asteroid::clock_time_update(self, msg);
            }
            BotRole::CargoTransport { .. } => {
                roles::cargo_transport::clock_time_update(self, msg);
            }
        }
    }
}

impl Handler<SageResponse> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: SageResponse, ctx: &mut Context<Self>) {
        let addr = ctx.address();
        match &self.role {
            BotRole::MineAsteroid { .. } => {
                roles::mine_asteroid::sage_response(self, msg, addr);
            }
            BotRole::CargoTransport { .. } => {
                roles::cargo_transport::sage_response(self, msg, addr);
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Tick(pub tokio::time::Duration);

impl Handler<Tick> for BotActor {
    type Result = ();

    fn handle(&mut self, msg: Tick, ctx: &mut Context<Self>) {
        // log::info!("Tick {:?}", msg.0);

        {
            if let (Some(db), Some(state), Some(data)) = (
                self.db.lock().ok(),
                Some(format!("{:#?}", self.fleet_state())),
                serde_json::to_string(&self.operation).ok(),
            ) {
                db.conn
                    .execute(
                        "INSERT OR REPLACE INTO bot_ops (pubkey, state, data) VALUES (?1, ?2, ?3)",
                        rusqlite::params![self.fleet.0.to_string(), state, data],
                    )
                    .ok();
            }
        }

        let addr = ctx.address();
        match &self.role {
            BotRole::MineAsteroid { .. } => {
                roles::mine_asteroid::tick(self, msg, addr);
            }
            BotRole::CargoTransport { .. } => {
                roles::cargo_transport::tick(self, msg, addr);
            }
        }
    }
}
