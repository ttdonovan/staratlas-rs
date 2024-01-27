use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::{signature::Signer, signer::null_signer::NullSigner},
    Client, Cluster,
};

use staratlas_galaxy::Galaxy;
use staratlas_marketplace::{typedefs::OrderSide, OrderAccount, ID as PROGRAM_ID};
use staratlas_utils_config as config;

use std::io;
use std::ops::Deref;
use std::str::FromStr;

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
    // Load the configuration from the environment variables
    let config = config::load_from_env();

    // Get the Solana RPC URL from the configuration
    let rpc_url = config
        .solana_rpc_url
        .ok_or(anyhow::anyhow!("RPC URL not found"))?;

    // Create a new wallet and payer
    let wallet = Pubkey::new_unique();
    let payer = NullSigner::new(&wallet);

    // Create a new Anchor client
    let client = Client::new(Cluster::Custom(rpc_url.clone(), rpc_url), &payer);

    // Create a new CSV writer
    let mut wtr = csv::Writer::from_writer(io::stdout());

    // Write the header row to the CSV file
    wtr.write_record(&[
        "SYMBOL",
        "NAME",
        "SIDE",
        "ATLAS_PRICE",
        "REMAIN_QTY",
        "ORIG_QTY",
    ])?;

    // Get the NFT resources from the Galaxy API
    let galaxy = Galaxy::new();
    let resources = galaxy.get_resources();

    // Process the open orders for each NFT resource and write the results to the CSV file
    for nft in resources {
        // Convert the asset mint string to a Pubkey
        let asset_mint = Pubkey::from_str(&nft.mint)?;

        // Get the open orders for the asset
        let entries = get_open_orders_for_asset(&client, &asset_mint)?;

        // Separate the buys and sells
        let mut buys = vec![];
        let mut sells = vec![];

        for (_pubkey, order) in entries {
            // Filter out orders that are not priced in ATLAS
            if &order.currency_mint.to_string() != "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx" {
                continue;
            }

            // Add the order to the buys or sells vector depending on the order side
            match order.order_side {
                OrderSide::Buy => buys.push(order),
                OrderSide::Sell => sells.push(order),
            };
        }

        // Sort the buys and sells by price
        buys.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        sells.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        // Take the top 5 buys and sells
        let buys: Vec<_> = buys.iter().map(|x| x.clone()).take(5).collect();
        let sells: Vec<_> = sells.iter().map(|x| x.clone()).take(5).collect();

        // Write the buys and sells to stdout
        for buy in buys {
            wtr.write_record(&[
                &nft.symbol,
                &nft.name,
                "BUY",
                &buy.price.to_string(),
                &buy.order_remaining_qty.to_string(),
                &buy.order_origination_qty.to_string(),
            ])?;
        }

        for sell in sells {
            wtr.write_record(&[
                &nft.symbol,
                &nft.name,
                "SELL",
                &sell.price.to_string(),
                &sell.order_remaining_qty.to_string(),
                &sell.order_origination_qty.to_string(),
            ])?;
        }
    }

    wtr.flush()?;

    Ok(())
}
