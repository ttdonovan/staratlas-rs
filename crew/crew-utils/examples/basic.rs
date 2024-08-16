use anchor_client::{
    solana_client::rpc_request::TokenAccountsFilter, solana_sdk::signer::null_signer::NullSigner,
    Client, Cluster,
};
use anchor_lang::{prelude::Pubkey, pubkey};
use mpl_token_metadata::accounts::Metadata;

/*
solana.fm/address/CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB?cluster=mainnet-alpha


solscan.io/tx/5p4nDWTvyFTmRShQjDdt6qaZAZbZcuHxPxUtAHhJ8YEQASSr44LGyMW6JniGBbkAqxuLHQfGXx3roE7pTAmhWu51
CREW #4433 - how to determine the asset id of a crew member?
solana.fm/address/BhK3zqFngtpcuQf8eEuwe8Eg6BdcaTDngKJmxmX5NnUq

galaxy.staratlas.com/crew/AFERAAA4OTI4ODM1NGFlMzQ2YWE4AAACAAEAAAAAAAAAAQAAAAAAAAABAAAAAAAAAAEAAAAAAAAAAQAAAAAAAAABAAAAAAAAAAEAAAAAAAAAAQAAAAAAAAA
https://galaxy.staratlas.com/crew?mintOffset=4433

*/

const COLLECTION_MINT: Pubkey = pubkey!("CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB");
const COLLECTION_METADATA: Pubkey = pubkey!("3vgLrm22BXcURo4TK2cqQkYBH3YaEYnVLE8tBpgEqkFg");

const MERKLE_TREE: Pubkey = pubkey!("treeT9umdMVUFGHXpGUzMGtJoCTHm11xHGWqjsemJqj"); // different each mint?
fn main() -> anyhow::Result<()> {
    let pubkey = mpl_bubblegum::utils::get_asset_id(&MERKLE_TREE, 4433);
    dbg!(&pubkey); // ANnGgRRnKfqhu8qQ3YmzktFb51mXtNBwnuWdDKEZTrMn

    let (pubkey, _bump) = Metadata::find_pda(&COLLECTION_MINT);
    assert_eq!(&pubkey, &COLLECTION_METADATA);

    // setup rpc client
    // let wallet = Pubkey::new_unique();
    let wallet = pubkey!("2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77");
    let payer = NullSigner::new(&wallet);
    let client = Client::new(Cluster::Mainnet, &payer);
    let program = client.program(mpl_bubblegum::ID)?;
    let rpc_client = program.rpc();

    // look-up Crew Collection Metadata
    let account = rpc_client.get_account(&pubkey)?;
    let metadata = Metadata::from_bytes(account.data.as_slice())?;
    // dbg!(&metadata);

    let crew = pubkey!("BhK3zqFngtpcuQf8eEuwe8Eg6BdcaTDngKJmxmX5NnUq");
    let result = rpc_client
        .get_token_accounts_by_owner(&wallet, TokenAccountsFilter::Mint(COLLECTION_MINT))?;
    dbg!(result);

    // // look-up CREW #4433
    // let crew = pubkey!("BhK3zqFngtpcuQf8eEuwe8Eg6BdcaTDngKJmxmX5NnUq");
    // let account = rpc_client.get_account(&crew)?; // account not found, but how to get metadata?
    // dbg!(&account);

    Ok(())
}
