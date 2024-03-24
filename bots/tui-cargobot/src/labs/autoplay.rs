use super::{txs, SageResponse};
use crate::{sage, Pubkey};

pub fn cargo_transport(
    sage: &sage::SageContext,
    fleet_id: &Pubkey,
    fleet: &sage::Fleet,
    state: &sage::FleetState,
    from_sector: [i64; 2],
    to_sector: [i64; 2],
    mint: &Pubkey,
) -> anyhow::Result<SageResponse> {
    match state {
        sage::FleetState::Idle(idle) => {
            match &idle.sector {
                [40, 30] => {
                    // TODO: [-40, 30] [0, -39]
                    let cargo_hold_full = txs::is_cargo_hold_at_capacity(sage, fleet)?;

                    if cargo_hold_full {
                        // returning with cargo...
                        txs::dock_to_starbase(sage, fleet_id, fleet, state, &idle)?;
                    } else {
                        // go-to other sector...
                        txs::warp_to_coordinate(sage, fleet_id, fleet, to_sector)?;
                    };

                    let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
                    return Ok(SageResponse::UpdateFleetState(fleet_state));
                }
                _ => {
                    let cargo_hold_full = txs::is_cargo_hold_at_capacity(sage, fleet)?;

                    if cargo_hold_full {
                        // return to from sector...
                        txs::warp_to_coordinate(sage, fleet_id, fleet, from_sector)?;
                    } else {
                        // pick up cargo...
                        txs::dock_to_starbase(sage, fleet_id, fleet, state, &idle)?;
                    }

                    let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
                    return Ok(SageResponse::UpdateFleetState(fleet_state));
                }
            }
        }
        sage::FleetState::MoveWarp(move_warp) => {
            let is_exit = txs::ready_to_exit_warp(sage, fleet_id, fleet, &move_warp)?;
            if is_exit {
                let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
                return Ok(SageResponse::ExitWarp(fleet_state));
            } else {
                return Ok(SageResponse::NoOperation);
            }
        }
        sage::FleetState::StarbaseLoadingBay(starbase_loading_bay) => {
            match starbase_loading_bay.starbase.to_string().as_str() {
                "J8aYFqhRnMmT5MUJg6JhBFUWJMty7VRTMZMpsJA56ttG" => {
                    let cargo_hold_full = txs::is_cargo_hold_at_capacity(sage, fleet)?;

                    if cargo_hold_full {
                        txs::withdraw_from_fleet_cargo_hold(
                            sage,
                            fleet_id,
                            &fleet,
                            &starbase_loading_bay.starbase,
                            mint,
                        )?;
                        return Ok(SageResponse::WithdrawFromFleetCargoHold);
                    } else {
                        txs::deposit_to_fleet_fuel_tank(
                            sage,
                            fleet_id,
                            &fleet,
                            &starbase_loading_bay.starbase,
                        )?;

                        txs::undock_from_starbase(sage, fleet_id, &fleet, state)?;

                        let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
                        return Ok(SageResponse::UpdateFleetState(fleet_state));
                    };
                }
                _ => {
                    let cargo_hold_full = txs::is_cargo_hold_at_capacity(sage, fleet)?;

                    if cargo_hold_full {
                        txs::undock_from_starbase(sage, fleet_id, fleet, state)?;
                    } else {
                        txs::deposit_to_fleet_cargo_hold(
                            sage,
                            fleet_id,
                            fleet,
                            &starbase_loading_bay.starbase,
                            mint,
                        )?;
                    }

                    let (_, fleet_state) = sage.fleet_with_state_accts(fleet_id)?;
                    return Ok(SageResponse::UpdateFleetState(fleet_state));
                }
            }
        }
        _ => {
            log::info!("[Sage Labs] - Unimplemented: {:?}", state);
            return Ok(SageResponse::NoOperation);
        }
    }
}
