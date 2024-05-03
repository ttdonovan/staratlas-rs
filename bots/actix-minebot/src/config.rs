use anchor_client::{
    solana_sdk::signature::{read_keypair_file, Keypair},
    Cluster,
};
use color_eyre::Result;
use dotenv::dotenv;
use serde::Deserialize;

use std::fs;
use std::rc::Rc;

pub struct Config {
    pub payer: Rc<Keypair>,
    pub cluster: Cluster,
    pub sage_bot_cfg: SageBotCfg,
}

#[derive(Deserialize)]
pub struct SageBotCfg {
    pub game_id: String,
    pub bots: Vec<BotCfg>,
}

#[derive(Deserialize)]
pub struct BotCfg {
    pub fleet_id: String,
    pub planet_id: String,
    pub mine_item_id: String,
}

pub fn init_config() -> Result<Config> {
    dotenv().ok();

    let path = std::env::var("PROVIDER_WALLET")?;
    let payer = read_keypair_file(&path).expect("Failed to read keypair file");
    let payer = Rc::new(payer);

    let url = std::env::var("PROVIDER_CLUSTER")?;
    let cluster = Cluster::Custom(url.clone(), url);

    let args: Vec<String> = std::env::args().collect();
    let sage_bot_cfg_path = &args[1];
    let sage_bot_cfg_json = fs::read_to_string(sage_bot_cfg_path)?;
    let sage_bot_cfg: SageBotCfg = serde_json::from_str(&sage_bot_cfg_json)?;

    Ok(Config {
        payer,
        cluster,
        sage_bot_cfg,
    })
}

pub fn init_logger() -> Result<()> {
    tui_logger::init_logger(log::LevelFilter::Info)?;
    tui_logger::set_default_level(log::LevelFilter::Info);
    log::info!("logging initialized");
    Ok(())
}
