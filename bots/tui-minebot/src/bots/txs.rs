use super::*;

pub fn sage_start_mining_asteroid(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.start_mining_asteroid(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;

            Ok(signature)
        }
        Err(err) => {
            // println!("Error: {:?}", err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_stop_mining_asteroid(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.stop_mining_asteroid(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;

            Ok(signature)
        }
        Err(err) => {
            // println!("Error: {:?}", err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_dock_to_starbase(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.dock_to_starbase(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            Ok(signature)
        }
        Err(err) => {
            // println!("Error: {:?}", err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_undock_from_starbase(
    bot: &mut MiningBot,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, state) = &bot.fleet;

    match sage.undock_from_starbase(fleet_id, fleet, state) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;
            bot.txs = Some(signature);
            bot.txs_counter += 1;
            Ok(signature)
        }
        Err(err) => {
            // println!("Error: {:?}", err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_mine_item_widthdraw_from_fleet(
    bot: &mut MiningBot,
    starbase: &Pubkey,
    sage: &sage::SageContext,
) -> Result<Option<Signature>, anyhow::Error> {
    let (fleet_id, _, _, fleet, _) = &bot.fleet;
    let mine_item = &bot.mine_item.2;
    let mint = &mine_item.0.mint;

    match sage.withdraw_from_fleet(
        fleet_id, fleet, starbase, mint, None, // withdraw all (max)
    ) {
        Ok(signature) => {
            bot.is_fleet_state_dirty = true;

            if let Some(signature) = signature {
                bot.txs = Some(signature);
                bot.txs_counter += 1;
                return Ok(Some(signature));
            }

            Ok(None)
        }
        Err(err) => {
            // println!("Error: {:?}", err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}

pub fn sage_deposit_to_fleet(
    bot: &mut MiningBot,
    starbase: &Pubkey,
    cargo_pod_to: &Pubkey,
    mint: &Pubkey,
    amount: u64,
    sage: &sage::SageContext,
) -> Result<Signature, anyhow::Error> {
    let (fleet_id, _, _, fleet, _) = &bot.fleet;

    match sage.deposit_to_fleet(fleet_id, fleet, &starbase, cargo_pod_to, mint, amount) {
        Ok(signature) => {
            bot.txs = Some(signature);
            bot.txs_counter += 1;

            bot.is_fleet_state_dirty = true;
            Ok(signature)
        }
        Err(err) => {
            // println!("Error: {:?}", err);
            bot.txs_errors += 1;
            Err(err)
        }
    }
}
