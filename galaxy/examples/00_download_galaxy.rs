use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let resp = reqwest::get("https://galaxy.staratlas.com/nfts").await?;

    let body = resp.text().await?;
    let data = serde_json::from_str::<serde_json::Value>(&body)?;
    let json = serde_json::to_string_pretty(&data)?;

    let mut output = File::create("galaxy/galaxy.json")?;
    write!(output, "{}", json)?;

    Ok(())
}
