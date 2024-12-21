# staratlas-rs

A collection of Rust crates to build on [Star Atlas](https://staratlas.com/).

See [Star Altas Builder: Resources for Builders](https://build.staratlas.com/).

## Rust Setup

See [rustup.rs](https://rustup.rs/) for toolchain installation.

## Development Setup

Copy `.env.sample` to `.env` and configure.

```
$ cp .env.sample .env
```

To generate crate documentation.

```
$ cargo doc --no-deps
# see 'target/doc'
```

## APIs and Data

A collection of utiltiy scripts for fetch data from Star Atlas APIs.

## Bots

A collection of various bots to interact with Star Atlas programs.

## Programs

### cargo

Solana IDL to work with SA cargo.

### claim

Solana IDL to work with SA claim.

### crafting

Solana IDL to work with SA carfting.

### player-profile

Solana IDL to work with SA player-profile.

See [player-profile/README.md](programs/player-profile/README.md) for example.

### profile-vault

Solana IDL to work with SA profile-vault.

### marketplace

Solana IDL to work with SA marketplace.

See [marketplace/README.md](programs/marketplace/README.md) for example.

### sage

Solana IDL to work with SA sage.

### score

Solana IDL to work with SA score.

## Program SDKs

These are extended SDKs around SA Programs.

## Crates

### galaxy

Rust data types defined/built from galaxy.staratlas.com/nfts JSON.

See [galaxy/README.md](galaxy/README.md) for more details.

## CLIs

### sa-market-cli: Star Atlas Marketplace CLI

A simple utility to dump "orders" from Marketplace to a CSV.

See [clis/marketplace-cli/README.md](clis/marketplace-cli/README.md) for usage.

```
cargo run -p sa-marketplace-cli -- --help
```

### sa-player-profile-cli: Star Atlas Player Profile CLI

A utility to manage (add/remove) "ProfileKey" on a SA Player Profile.

```
cargo run -p sa-palyer-profile -- --help
```

### sa-sage-cli: Star Atlas Sage CLI (WIP)

A CLI program to interact with SA Sage.

```
cargo run -p sa-sage-cli -- --help
```

See [clis/sage-cli/README.md](clis/sage-cli/README.md) for usage.

## Utils

### config

A simple utility to load common configruation variables from an ENV.

```
use staratlas_utils_config as config;

let config = config::load_from_env();
```

See `.env.sample`.

### wallet

A simple utility to encrypt/decrypt a Solana keypair in Rust.

See [utils/wallet-rs/README.md](utils/wallet-rs/README.md) for examples.