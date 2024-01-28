use anchor_client::{
    anchor_lang::prelude::Pubkey,
    solana_client::rpc_filter::{Memcmp, RpcFilterType},
    solana_sdk::{signature::Signer, signer::null_signer::NullSigner},
    Client, Cluster,
};
use clap::{Parser, Subcommand, ValueEnum};
use dotenv::dotenv;

use staratlas_galaxy::Galaxy;
use staratlas_marketplace::{typedefs::OrderSide, OrderAccount, ID as PROGRAM_ID};

use std::fs::File;
use std::ops::Deref;
use std::str::FromStr;

/// Star Atlas: Marketplace CLI --> donations: 2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77 <--
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the open orders for each NFT resource to a CSV file
    #[command(arg_required_else_help = true)]
    Dump {
        /// The path to the CSV file to write
        output: String,
        /// Solana RCP URL, if none is provided the cli will attempt to read from the SOLANA_RPC_URL environment variable
        #[arg(long, value_name = "SOLANA_RPC_URL")]
        rpc_url: Option<String>,
        /// The currency of price for orders to dump
        #[arg(short, long, value_enum, default_value = "atlas")]
        currency: Currency,
        /// The depth of the order book to dump
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..10), default_value = "3")]
        depth: u8,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Currency {
    ATLAS,
    USDC,
}

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

fn run(output: &str, currency: &Currency, rpc_url: &str, depth: &u8) -> anyhow::Result<()> {
    // Create a new wallet and payer
    let wallet = Pubkey::new_unique();
    let payer = NullSigner::new(&wallet);

    // Create a new Anchor client
    let client = Client::new(
        Cluster::Custom(rpc_url.to_string(), rpc_url.to_string()),
        &payer,
    );

    // Create a new CSV writer
    let file = File::create(output)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Write the header row to the CSV file
    wtr.write_record(&[
        "SYMBOL",
        "NAME",
        "SIDE",
        "CURRENCY",
        "CURRENCY_PRICE",
        "REMAIN_QTY",
        "ORIG_QTY",
    ])?;

    // Get the NFT resources from the Galaxy API
    let galaxy = Galaxy::new();
    let resources = galaxy.get_resources();

    let currency_mint = match currency {
        Currency::ATLAS => "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx",
        Currency::USDC => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    };

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
            // Filter out orders that are not priced in "currency_mint"
            if &order.currency_mint.to_string() != currency_mint {
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

        // Take the top "depth" buys and sells
        let buys: Vec<_> = buys
            .iter()
            .map(|x| x.clone())
            .take(*depth as usize)
            .collect();
        let sells: Vec<_> = sells
            .iter()
            .map(|x| x.clone())
            .take(*depth as usize)
            .collect();

        // Write the buys and sells to stdout
        for buy in buys {
            wtr.write_record(&[
                &nft.symbol,
                &nft.name,
                "BUY",
                &format!("{:?}", currency),
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
                &format!("{:?}", currency),
                &sell.price.to_string(),
                &sell.order_remaining_qty.to_string(),
                &sell.order_origination_qty.to_string(),
            ])?;
        }
    }

    wtr.flush()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Dump {
            output,
            currency,
            rpc_url,
            depth,
        } => {
            let rpc_url = rpc_url.clone().unwrap_or_else(|| {
                dotenv().ok();
                dotenv::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set")
            });

            run(&output, currency, &rpc_url, depth)?;
        }
    };

    Ok(())
}
