# Notes

* https://docs.solana.com/cli/install
* https://github.com/mercurial-finance/vault-sdk

* https://github.com/ImGroovin/Lab-Assistant/tree/main/Freelance_System_Core
* https://docs.google.com/document/d/1FQZl7UOXdj64vgc5rCVicPtDG7R2gl-lHEX3_bnWvKU/edit?pli=1

## Phantom Wallet Address with Solana CLI

Install [Solana CLI](https://docs.solana.com/cli/install).

```
# generate a file system wallet
$ solana-keygen recover 'prompt:?key=0/0' --outfile ~/.config/solana/id.json
```

```
# enter 'Secret Recovery Phrase' from Phantom Wallet
```

```
# check wallet balance
$ solana balance
```

```
# view wallet address (public key)
$ solana-keygen pubkey
> abc123
```

```
# verify public key matches private key
$ solana-keygen verify abc123
> Verification for public key: abc123: Success
```

## RPC

```
    // Used for reading solana data
    let readRPCs = [
        'https://rpc.hellomoon.io/57dbc69d-7e66-4454-b33e-fa6a4b46170f', // Hello Moon
        'https://staratl-mainc06-2415.mainnet.rpcpool.com', // Triton
        'https://mainnet.helius-rpc.com/?api-key=735486d8-ae86-4d26-829c-e34a2210d119', // Helius
        'https://twilight-autumn-diagram.solana-mainnet.quiknode.pro/4fc53d638efd1cc0f80764bc457944bb325d1ff1', // Quicknode
        'https://solana-api.syndica.io/access-token/WPoEqWQ2auQQY1zHRNGJyRBkvfOLqw58FqYucdYtmy8q9Z84MBWwqtfVf8jKhcFh/rpc', // Syndica (Old)
    ];

    // Used for pushing transactions to solana chain
    let writeRPCs = [
        'https://twilight-autumn-diagram.solana-mainnet.quiknode.pro/4fc53d638efd1cc0f80764bc457944bb325d1ff1', // Quicknode
        'https://rpc.hellomoon.io/57dbc69d-7e66-4454-b33e-fa6a4b46170f', // Hello Moon
        'https://staratl-mainc06-2415.mainnet.rpcpool.com', // Triton
        'https://mainnet.helius-rpc.com/?api-key=735486d8-ae86-4d26-829c-e34a2210d119', // Helius
        'https://solana-api.syndica.io/access-token/WPoEqWQ2auQQY1zHRNGJyRBkvfOLqw58FqYucdYtmy8q9Z84MBWwqtfVf8jKhcFh/rpc', // Syndica (Old)
    ];
```