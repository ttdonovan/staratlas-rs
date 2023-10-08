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

## Crates

### cargo

Solana IDL to work with SA cargo.

### claim

Solana IDL to work with SA claim.

### crafting

Solana IDL to work with SA carfting.

### galaxy

Rust data types defined/built from galaxy.staratlas.com/nfts JSON.

See [galaxy/README.md](galaxy/README.md) for more details.

### marketplace

Solana IDL to work with SA marketplace.

See [marketplace/README.md](marketplace/README.md) for example.

### player-profile

Solana IDL to work with SA player-profile.

See [player-profile/README.md](player-profile/README.md) for example.

### player-vault

Solana IDL to work with SA player-valut.

### sage

`FIXME`

### score

Solana IDL to work with SA score.

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