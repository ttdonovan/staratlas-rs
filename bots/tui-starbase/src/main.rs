use crossterm::event::KeyCode;

use shared_sage_commander as commander;

mod errors;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    errors::init_hooks().unwrap();

    let from_sector = [40, 30];
    dbg!(&from_sector);

    let mut events = tui::EventHandler::new();
    let (tx, rx, _task) = commander::sage_commander_sender_and_receiver();

    loop {
        tokio::select! {
            maybe_event = events.next() => {
                if let Ok(event) = maybe_event {
                    match event {
                        tui::Event::Key(key) => {
                            match key.code {
                                KeyCode::Char('q') => {
                                    break;
                                },
                                KeyCode::Char('f') => {
                                    let cmd = commander::Command::Find(commander::Find::Planets(
                                        from_sector
                                    ));
                                    dbg!(&cmd);

                                    tx.send(cmd)?;
                                },
                                _ => {}
                            }
                        }
                        tui::Event::Tick => {
                            // dbg!("tick");

                            if let Some(reply) = rx.try_recv().ok()  {
                                dbg!(&reply);
                            }
                        },
                        tui::Event::Error => {
                            break;
                        }
                    }
                }
            },
        }
    }

    Ok(())
}
