use color_eyre::{eyre::eyre, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};

use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Error,
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    _tx: mpsc::UnboundedSender<Event>,
    rx: mpsc::UnboundedReceiver<Event>,
    _task: Option<JoinHandle<()>>,
}

impl EventHandler {
    pub fn new() -> Self {
        let tick_rate = Duration::from_millis(250);

        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        let task = tokio::spawn(async move {
            let mut rdr = EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);

            loop {
                let delay = interval.tick();
                let crossterm_event = rdr.next().fuse();

                tokio::select! {
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(evt)) => {
                                match evt {
                                    CrosstermEvent::Key(key) => {
                                        if key.kind == KeyEventKind::Press {
                                            tx.send(Event::Key(key)).unwrap();
                                        }
                                    },
                                    _ => { },
                                }
                            },
                            Some(Err(_)) => {
                                tx.send(Event::Error).unwrap();
                            },
                            None => {},
                        }
                    },
                    _ = delay => {
                        tx.send(Event::Tick).unwrap();
                    }
                }
            }
        });

        EventHandler {
            _tx,
            rx,
            _task: Some(task),
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx.recv().await.ok_or(eyre!("Unable to get event"))
    }
}
