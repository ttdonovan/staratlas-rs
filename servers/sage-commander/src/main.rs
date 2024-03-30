use color_eyre::Result;
use futures_util::{FutureExt, SinkExt, StreamExt};
use warp::Filter;

use shared_sage_cli as cli;
use shared_sage_commander as cmds;
use shared_sage_context as sage_ctx;

mod errors;
mod sage;

#[tokio::main]
async fn main() -> Result<()> {
    errors::init_hooks()?;

    let echo = warp::path("echo").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| {
            let (tx, rx) = websocket.split();

            rx.forward(tx).map(|result| {
                if let Err(e) = result {
                    eprintln!("websocket error: {}", e);
                }
            })
        })
    });

    let cmd = warp::path("cmd").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| async {
            use crate::cmds::Command;

            dbg!("sage_handler");
            let sage_handler = sage::init();

            let (mut tx, mut rx) = websocket.split();

            let tick_rate = std::time::Duration::from_millis(250);
            let mut interval = tokio::time::interval(tick_rate);

            loop {
                let delay = interval.tick();
                let command_rx = rx.next().fuse();

                tokio::select! {
                    maybe_command = command_rx => {
                        if let Some(Ok(message)) = maybe_command {
                            let data = message.as_bytes();
                            dbg!(data.len());

                            if data.len() != 0 {
                                let cmd = borsh::from_slice::<Command>(data).unwrap();
                                sage_handler.send(cmd).unwrap();
                            }
                        }
                    },
                    _ = delay => {
                        if let Some(response) = sage_handler.poll_response() {
                            use sage::SageResponse;

                            let reply = match response {
                                SageResponse::Fleet(fleet) => {
                                    cmds::Reply::Fleet(fleet)
                                },
                                SageResponse::FleetState(fleet_state) => {
                                    cmds::Reply::FleetState(fleet_state)
                                },
                                SageResponse::FleetWithState(fleet_with_state) => {
                                    cmds::Reply::FleetWithState(fleet_with_state)
                                },
                                SageResponse::Planets(planets) => {
                                    cmds::Reply::Planets(planets)
                                },
                            };

                            let encoded = borsh::to_vec(&reply).unwrap();
                            let message = warp::ws::Message::binary(encoded);
                            tx.send(message).await.unwrap();
                        }
                    }
                }
            }
        })
    });

    let routes = echo.or(cmd);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
