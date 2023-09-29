# staratlas-rs

A collection of Rust crates to build on Star Atlas

## Development

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

### claim

Solana IDL to work with SA claim.

### galaxy

Rust data types defined/built from galaxy.staratlas.com/nfts JSON.

See [galaxy/README.md](galaxy/README.md) for more details.

### marketplace

Solana IDL to work with SA marketplace.

See [marketplace/README.md](marketplace/README.md) for example.

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