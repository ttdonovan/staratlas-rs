use std::fs::File;
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use galaxy_api::GALAXY_API_NFTS;

#[tokio::main]
async fn main() -> Result<()> {
    let resp = reqwest::get(GALAXY_API_NFTS).await?;
    let body = resp.text().await?;

    let mut output = File::create("data/galaxy.json")?;
    write!(output, "{}", body)?;

    Ok(())
}
