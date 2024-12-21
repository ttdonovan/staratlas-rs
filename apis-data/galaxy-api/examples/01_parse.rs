use std::env;
use std::fs::File;

use galaxy_api::types::{Items, ItemType};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let nfts: Items = serde_json::from_reader(file)?;

    // dbg!(nfts);
    for item in nfts.0.iter() {
        if item.attributes.item_type != ItemType::Resource {
            continue;
        }

        dbg!(&item);
    }

    Ok(())
}