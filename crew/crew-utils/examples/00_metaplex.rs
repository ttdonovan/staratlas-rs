use dotenv::dotenv;
use mpl_core::{accounts::BaseAssetV1, types::Key, Asset, Collection, ID as MPL_CORE_ID};
use solana_client::{
    nonblocking::rpc_client,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::pubkey::Pubkey;

use std::str::FromStr;

// https://developers.metaplex.com/core/fetch

// Fetch a Single Asset
pub async fn fetch_asset(
    rpc_client: &rpc_client::RpcClient,
    asset_id: &Pubkey,
) -> anyhow::Result<Asset> {
    let rpc_data = rpc_client.get_account_data(&asset_id).await?;
    let asset = Asset::from_bytes(&rpc_data)?;
    Ok(*asset)
}

// Fetch a Core Collection
pub async fn fetch_collection(
    rpc_client: &rpc_client::RpcClient,
    collection_id: &Pubkey,
) -> anyhow::Result<Collection> {
    let rpc_data = rpc_client.get_account_data(&collection_id).await?;
    let collection = Collection::from_bytes(&rpc_data)?;
    Ok(*collection)
}

// Fetch Assets by Owner
pub async fn fetch_assets_by_owner(
    rpc_client: &rpc_client::RpcClient,
    owner: &Pubkey,
) -> anyhow::Result<Vec<BaseAssetV1>> {
    let rpc_data = rpc_client
        .get_program_accounts_with_config(
            &MPL_CORE_ID,
            RpcProgramAccountsConfig {
                filters: Some(vec![
                    RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![Key::AssetV1 as u8]),
                    )),
                    RpcFilterType::Memcmp(Memcmp::new(
                        1,
                        MemcmpEncodedBytes::Base58(owner.to_string()),
                    )),
                ]),
                account_config: RpcAccountInfoConfig {
                    encoding: None,
                    data_slice: None,
                    commitment: None,
                    min_context_slot: None,
                },
                with_context: None,
            },
        )
        .await?;

    let accounts_iter = rpc_data.into_iter().map(|(_, account)| account);

    let mut assets: Vec<BaseAssetV1> = vec![];

    for account in accounts_iter {
        let asset = BaseAssetV1::from_bytes(&account.data)?;
        assets.push(asset);
    }

    Ok(assets)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let rpc_url = dotenv::var("SOLANA_RPC_ENDPOINT").unwrap_or_else(|_| "https://api.devnet.solana.com".into());
    // dbg!(&rpc_url);

    let rpc_client = rpc_client::RpcClient::new(rpc_url);

    // let asset_id = Pubkey::from_str("BhK3zqFngtpcuQf8eEuwe8Eg6BdcaTDngKJmxmX5NnUq")?;
    // let asset = fetch_asset(&rpc_client, &asset_id).await?;
    // dbg!(&asset); // AccountNotFound: pubkey=BhK3zqFngtpcuQf8eEuwe8Eg6BdcaTDngKJmxmX5NnUq

    // let collection_id = Pubkey::from_str("CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB")?;
    // let collection = fetch_collection(&rpc_client, &collection_id).await?;
    // dbg!(&collection); // Error: Unexpected length of input

    let owner = Pubkey::from_str("2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77")?;
    let assets = fetch_assets_by_owner(&rpc_client, &owner).await?;
    dbg!(&assets); // []

    Ok(())
}
