use super::*;

mod autoplay;

mod tick;
pub use tick::*;

mod response;

pub struct Bot {
    fleet_id: Pubkey,
    pub(crate) mine_args: (Pubkey, Pubkey, Pubkey), // (Planet, MineItem, Mint)
    pub(crate) fleet: Option<Fleet>,
    pub(crate) fleet_state: Option<FleetState>,
    pub(crate) fleet_cargo_hold: Vec<(String, u64)>,
    pub(crate) fleet_fuel_tank: Vec<(String, u64)>,
    pub(crate) fleet_ammo_bank: Vec<(String, u64)>,
    pub(crate) fleet_food_cargo: Vec<(String, u64)>,
    pub(crate) planet: Option<Planet>,
    resource_id: Option<Pubkey>,
    pub(crate) resource: Option<Resource>,
    pub(crate) mine_item: Option<MineItem>,
    pub(crate) mine_item_mint: Option<Pubkey>,
    clock: Option<Clock>,
    pub(crate) addr_sage: Addr<SageBased>,
    pub(crate) operation: Option<autoplay::BotOps>,
}

impl Bot {
    pub fn new(
        fleet_id: Pubkey,
        mine_args: (Pubkey, Pubkey, Pubkey), // (Planet, MineItem, Mint)
        addr_sage: Addr<SageBased>,
    ) -> Self {
        Self {
            fleet_id,
            mine_args,
            fleet: None,
            fleet_state: None,
            fleet_cargo_hold: vec![],
            fleet_fuel_tank: vec![],
            fleet_ammo_bank: vec![],
            fleet_food_cargo: vec![],
            planet: None,
            resource_id: None,
            resource: None,
            mine_item: None,
            mine_item_mint: None,
            clock: None,
            addr_sage,
            operation: None,
        }
    }
}

impl Actor for Bot {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping(pub Option<Signature>);

impl Handler<Ping> for Bot {
    type Result = ();

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) {
        log::info!("Pong: {:?}", msg.0);
        self.operation = None; // Clear operation
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClockTimeUpdate(pub Clock);

impl Handler<ClockTimeUpdate> for Bot {
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
