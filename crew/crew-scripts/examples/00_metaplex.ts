import { createUmi, Umi } from '@metaplex-foundation/umi-bundle-defaults';
import { dasApi } from '@metaplex-foundation/digital-asset-standard-api';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

// console.log("00_metaplex.ts");

const crewCollectionId = "CREWSAACJTKHKhZi96pLRJXsxiGbdZaQHdFW9r7qGJkB";

const argv = yargs(hideBin(process.argv))
  .option('owner', {
    type: 'string',
    description: 'The owner wallet address',
    default: process.env.SOLANA_WALLET_ADDRESS || '1111111111111111111111111111111111111111111'
  })
  .help()
  .argv;

const rpcEndpoint: string = process.env.SOLANA_RPC_ENDPOINT || 'https://api.devnet.solana.com';

const umi = createUmi(rpcEndpoint).use(dasApi());

async function main() {
    const umi: Umi = createUmi(rpcEndpoint).use(dasApi());

    const assets = await umi.rpc.getAssetsByOwner({
        owner: argv.owner,
        limit: 1000,
        page: 1
    });

    const crewAssets = assets.items.filter((asset: any) => asset.grouping[0]?.group_value == crewCollectionId);
    // console.log(crewAssets.length);

    const crewNumbers = crewAssets.map((crew: any) => {
        // console.log(crew);
        const num = crew.content.metadata.name.replace(/^CREW #/, "");
        return parseInt(num, 10);
    }).filter((num: number) => !isNaN(num));

    const sortedCrewNumbers = crewNumbers.sort((a: number, b: number) => a - b);

    const data = sortedCrewNumbers.join('\n')
    console.log(data);
}

main().catch(console.error);