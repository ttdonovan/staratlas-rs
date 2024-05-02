use anchor_lang::prelude::Pubkey;
use tokio::signal;
use tokio::time::Duration;

use shared_sage_cli as cli;
use shared_sage_context as sage;
use shared_time as time;

use std::sync::Mutex;
use std::thread;

static BOTS: Mutex<Vec<Bot>> = Mutex::new(Vec::new());
static FLEET_IDS: Mutex<Vec<Pubkey>> = Mutex::new(Vec::new());
static FLEETS: Mutex<Vec<sage::Fleet>> = Mutex::new(Vec::new());
static FLEET_STATES: Mutex<Vec<sage::FleetState>> = Mutex::new(Vec::new());

#[derive(Debug)]
struct Bot {
    idx: usize,
}

impl Bot {
    // fn fleet(&self) -> sage::Fleet {
    //     let fleets = FLEETS.lock().unwrap();
    //     let fleet = fleets[self.idx];
    //     fleet
    // }

    // fn fleet_state(&self) -> sage::FleetState {
    //     let fleet_states = FLEET_STATES.lock().unwrap();
    //     let fleet_state = fleet_states[self.idx];
    //     fleet_state
    // }
}

async fn task_one() {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("task (1) tick");

                let fleets = FLEETS.lock().unwrap();
                dbg!(fleets.len());

                let fleet_states = FLEET_STATES.lock().unwrap();
                dbg!(&fleet_states);
            }
        }
    }
}

async fn task_two(tx: std::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(20));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("task (2) tick");
                tx.send("FleetState".to_string())?;
            }
        }
    }
}

fn sage_client_background_thread(rx: std::sync::mpsc::Receiver<String>) -> anyhow::Result<()> {
    let cli = cli::cli_parse();
    let client = cli::init_client(&cli)?;
    let (game_id, _) = cli::init_sage_config(&cli);
    let sage = sage::SageContext::new(&client, &game_id)?;

    // one-time setup
    {
        let fleet_ids = FLEET_IDS.lock().unwrap();
        let mut fleets = FLEETS.lock().unwrap();
        let mut fleet_states = FLEET_STATES.lock().unwrap();
        let mut bots = BOTS.lock().unwrap();

        for (idx, fleet_id) in fleet_ids.iter().enumerate() {
            let (fleet, state) = sage.fleet_with_state_accts(&fleet_id)?;
            fleets.push(fleet);
            fleet_states.push(state);
            bots.push(Bot { idx });
        }
    }

    // wait for messages
    while let Some(msg) = rx.recv().ok() {
        println!("Received: {}", msg);
        let fleet_ids = FLEET_IDS.lock().unwrap();
        let mut fleets = FLEETS.lock().unwrap();
        let mut fleet_states = FLEET_STATES.lock().unwrap();

        for (idx, fleet_id) in fleet_ids.iter().enumerate() {
            let (fleet, state) = sage.fleet_with_state_accts(&fleet_id)?;
            fleets[idx] = fleet;
            fleet_states[idx] = state;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::cli_parse();
    let (game_id, fleet_ids) = cli::init_sage_config(&cli);

    let mut ids = FLEET_IDS.lock().unwrap();
    *ids = fleet_ids.clone();
    drop(ids);

    dbg!(&game_id);
    // dbg!(&fleet_ids);

    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let _thread = thread::spawn(move || sage_client_background_thread(rx));

    let _task1 = tokio::spawn(task_one());
    let _task2 = tokio::spawn(task_two(tx));

    let ctrl_c = signal::ctrl_c();
    println!("Press Ctrl+C to exit...");
    ctrl_c.await?;

    println!("Exiting...");

    Ok(())
}
