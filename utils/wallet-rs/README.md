# wallet

Goal or purpose of this util is to create an encrypted file that keeps a Solana
keypair safe from prying eyes.

## Usage

Creates a "main" (alias) wallet at `tmp/wallet.enc`.

```
cargo run -p staratlas-utils-wallet --example 01_write_wallet
cargo run -p staratlas-utils-wallet --example 02_read_wallet
cargo run -p staratlas-utils-wallet --example 03_rotate_wallet
```

## Hack Me

Take the funds and tokens if you can.

https://solscan.io/account/2yodqKtkdNJXxJv21s5YMVG8bjscaezLVFRfnWra5D77

See `hackme.wallet.enc`.