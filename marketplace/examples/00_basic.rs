use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::{signature::Signer, signer::null_signer::NullSigner},
    Client, Cluster,
};

use staratlas_galaxy::Galaxy;
use staratlas_marketplace::{OrderAccount, ID as PROGRAM_ID};

use std::ops::Deref;
use std::str::FromStr;

const RPC_URL: &str = "replace-with-your-rpc-url";

pub fn get_open_orders_for_asset<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    asset_mint: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, OrderAccount)>> {
    let program = client.program(PROGRAM_ID);

    let orders = program.accounts::<OrderAccount>(vec![
        RpcFilterType::DataSize(201),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(72, &asset_mint.to_bytes())),
    ])?;

    Ok(orders)
}

fn main() -> anyhow::Result<()> {
    // Star Atlas NFTs
    let galaxy = Galaxy::new();
    let nft = galaxy
        .find_symbol("TOOL")
        .ok_or(anyhow::anyhow!("NFT not found"))?;

    dbg!(nft);

    // setup Anchor client
    let wallet = Pubkey::new_unique();
    let payer = NullSigner::new(&wallet);
    let client = Client::new(Cluster::Custom(RPC_URL.into(), RPC_URL.into()), &payer);

    // get open orders for the asset
    let asset_mint = Pubkey::from_str(&nft.mint)?;
    let entries = get_open_orders_for_asset(&client, &asset_mint)?;

    // iterate over the entries and print them
    for entry in entries {
        let (_pubkey, order) = entry;

        dbg!(order.order_side);
        dbg!(order.price);
        dbg!(order.order_remaining_qty);
        dbg!(order.order_origination_qty);
    }

    Ok(())
}
