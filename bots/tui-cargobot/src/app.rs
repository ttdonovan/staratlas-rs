use std::time::{Duration, Instant};

use crate::{bots, labs, time, Pubkey, MAX_CARGO_RUNS};

pub fn init(game_id: Pubkey, fleet_id: Pubkey, bot_args: bots::BotArgs) -> anyhow::Result<App> {
    let sage = labs::SageHandler::new(game_id.clone());
    let app = App::new(sage, game_id, fleet_id, bot_args);
    app.refresh_fleet();

    Ok(app)
}

pub struct App {
    mode: Mode,
    pub stopwatch: time::Stopwatch,
    dt: Duration,
    last_time: Instant,
    sage: labs::SageHandler,
    pub game_id: Pubkey,
    fleet_id: Pubkey,
    bot_args: bots::BotArgs,
    pub bot: Option<bots::Bot>,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

impl App {
    pub fn new(
        sage: labs::SageHandler,
        game_id: Pubkey,
        fleet_id: Pubkey,
        bot_args: bots::BotArgs,
    ) -> Self {
        App {
            mode: Mode::default(),
            stopwatch: time::Stopwatch::default(),
            dt: Duration::ZERO,
            last_time: Instant::now(),
            sage,
            game_id,
            fleet_id,
            bot_args,
            bot: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    pub fn tick(&mut self) {
        // calculate delta time
        let now = Instant::now();
        self.dt = now.duration_since(self.last_time);
        self.last_time = now;

        // update app timers
        self.stopwatch.tick(self.dt);

        // update bot timers and send autoplay request to SAGE Labs
        if let Some(bot) = &mut self.bot {
            bot.timers.tick(self.dt);

            // only send if not waiting for a response
            if !bot.is_tx && bot.is_warp_cool_down_finished() {
                bot.is_tx = true;
                let autoplay = (bot.fleet_id, bot.fleet, bot.fleet_state.clone(), self.bot_args);
                let req = labs::SageRequest::AutoplayCargoTransport(autoplay);
                self.send_sage_request(req);
            }
        }

        // poll for responses from SAGE Labs
        if let Some(resp) = self.next_sage_response() {
            self.handle_sage_response(resp);
        }

        // check if app should quit
        if let Some(bot) = &self.bot {
            if bot.num_runs >= MAX_CARGO_RUNS {
                self.quit();
            }
        }
    }

    pub fn handle_sage_response(&mut self, resp: labs::SageResponse) {
        use labs::SageResponse;

        match resp {
            SageResponse::FleetWithState((fleet, fleet_state)) => {
                match &mut self.bot {
                    Some(bot) => {
                        bot.fleet = fleet;
                        bot.fleet_state = fleet_state;
                        log::info!("[Bot Refreshed] - {:?}", &self.fleet_id);
                    }
                    None => {
                        // if there is none bot, create one
                        let warp_cool_down =
                            Duration::from_secs(fleet.stats.movement_stats.warp_cool_down as u64);
                        let mut warp_cool_down_timer = time::Timer::new(warp_cool_down);
                        warp_cool_down_timer.set_elapsed(warp_cool_down); // set to finished state

                        let timers = bots::BotTimers {
                            warp_cool_down: warp_cool_down_timer,
                        };

                        let bot = bots::Bot {
                            fleet_id: self.fleet_id.clone(),
                            fleet,
                            fleet_state,
                            from_sector: self.bot_args.0,
                            to_sector: self.bot_args.1,
                            mint: self.bot_args.2.clone(),
                            timers,
                            num_runs: self.bot_args.3,
                            is_tx: false,
                        };

                        self.bot = Some(bot);
                        log::info!("[Bot Initialized] - {:?}", &self.fleet_id);
                    }
                }
            }
            SageResponse::UpdateFleetState(fleet_state) => {
                if let Some(bot) = &mut self.bot {
                    bot.fleet_state = fleet_state;
                    bot.is_tx = false;
                }
            }
            SageResponse::ExitWarp(fleet_state) => {
                if let Some(bot) = &mut self.bot {
                    bot.timers.warp_cool_down.reset();
                    bot.fleet_state = fleet_state;
                    bot.is_tx = false;
                }
            }
            SageResponse::WithdrawFromFleetCargoHold => {
                if let Some(bot) = &mut self.bot {
                    bot.num_runs += 1;
                    bot.is_tx = false;
                }
            }
            SageResponse::NoOperation => {
                if let Some(bot) = &mut self.bot {
                    bot.is_tx = false;
                }
            }
        }
    }

    pub fn quit(&mut self) {
        self.mode = Mode::Quit;
    }

    pub fn refresh_fleet(&self) {
        let req = labs::SageRequest::FleetWithState(self.fleet_id);
        self.send_sage_request(req);
    }

    pub fn send_sage_request(&self, req: labs::SageRequest) {
        self.sage.req_tx.send(req).unwrap();
    }

    pub fn next_sage_response(&self) -> Option<labs::SageResponse> {
        self.sage.resp_rx.try_recv().ok()
    }
}
