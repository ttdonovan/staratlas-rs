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

## Research

### Geyser

* https://crates.io/crates/solana-geyser-plugin-interface
* https://github.com/solana-labs/solana-accountsdb-plugin-postgres