use super::*;

mod action;
pub use action::*;

mod request;
pub use request::*;

pub struct SageBasedActor {
    client: Client<Rc<Keypair>>,
    payer: Rc<Keypair>,
    game_id: Pubkey,
    game: Game,
    subscribers: Vec<Recipient<ClockTimeUpdate>>,
}

impl SageBasedActor {
    pub fn new(
        client: Client<Rc<Keypair>>,
        payer: Rc<Keypair>,
        game_id: Pubkey,
        game: Game,
    ) -> Self {
        SageBasedActor {
            client,
            payer,
            game_id,
            game,
            subscribers: vec![],
        }
    }
}

impl Actor for SageBasedActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("SageBased Actor started...");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BlockHeight;

impl Handler<BlockHeight> for SageBasedActor {
    type Result = ();

    fn handle(&mut self, _: BlockHeight, ctx: &mut Context<Self>) -> Self::Result {
        let program = self.client.program(SAGE_ID).unwrap();
        let rpc = program.async_rpc();

        let fut = Box::pin(async move {
            let block_height = rpc.get_block_height().await.unwrap();
            log::info!("BlockHeight {}", block_height);
        });

        let actor_future = fut.into_actor(self);

        ctx.wait(actor_future);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SubscribeClockTime(pub Recipient<ClockTimeUpdate>);

impl Handler<SubscribeClockTime> for SageBasedActor {
    type Result = ();

    fn handle(&mut self, msg: SubscribeClockTime, _: &mut Self::Context) {
        log::info!("{:?}", msg.0);
        self.subscribers.push(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClockTime;

impl Handler<ClockTime> for SageBasedActor {
    type Result = ();

    fn handle(&mut self, _: ClockTime, ctx: &mut Context<Self>) -> Self::Result {
        use anchor_client::anchor_lang::solana_program::sysvar;

        let subscribers = self.subscribers.clone();
        let program = self.client.program(SAGE_ID).unwrap();
        let rpc = program.async_rpc();

        let fut = Box::pin(async move {
            let account = rpc.get_account(&sysvar::clock::id()).await.unwrap();
            let clock = account.deserialize_data::<Clock>().unwrap();

            for subscr in subscribers {
                subscr.do_send(ClockTimeUpdate(clock.clone()));
            }
        });

        let actor_future = fut.into_actor(self);

        ctx.wait(actor_future);
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct ClockTimeRequest(pub Addr<BotActor>);

impl Handler<ClockTimeRequest> for SageBasedActor {
    type Result = ();

    fn handle(&mut self, msg: ClockTimeRequest, ctx: &mut Context<Self>) -> Self::Result {
        use anchor_client::anchor_lang::solana_program::sysvar;
        let addr_bot = msg.0;

        let program = self.client.program(SAGE_ID).unwrap();
        let rpc = program.async_rpc();

        let fut = Box::pin(async move {
            let account = rpc.get_account(&sysvar::clock::id()).await.unwrap();
            let clock = account.deserialize_data::<Clock>().unwrap();

            addr_bot.do_send(ClockTimeUpdate(clock.clone()));
        });

        let actor_future = fut.into_actor(self);

        ctx.spawn(actor_future);
    }
}
