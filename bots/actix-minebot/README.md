# actix-minebot

A Sage Based bot that performs mine asteroid operations.

**Note:** manually run/step through the "mining operation" loop for a fleet/starbase to ensure all Associated Token accounts are created.

## Usage

Set environment variables (`export ...='...'` or `$env:...='...'`):

```
export PROVIDER_CLUSTER='https://mainnet.helius-rpc.com/?api-key=...'
export PROVIDER_WALLET='path/to/id.json'
```

```
# cp bots/actix-minebot/minebot-config.json.sample path/to/minebot-config.json
# edit 'path/to/minebot-config.json' as needed (current example of UST-1-3, Hydrogen)
cargo run --release -p actix-minebot -- path/to/minebot-config.json
```

### Example of Bot Roles

```
{
    "fleet_id": "11111111111111111111111111111111111111111111",
    "role": {
        "MineAsteroid": {
            "planet_id": "7jWrQYjfuHyQXVWfMyLireeukSpva99FvCLLxERCvT4U",
            "mine_item_id": "FpTUZKuviuGaww6ijjXdoeuJtFeEjabEXnzxRYHukhMx"
        }
    }
}
```


```
{
    "fleet_id": "11111111111111111111111111111111111111111111",
    "role": {
        "CargoTransport": {
            "cargo_mint": "MASS9GqtJz6ABisAxcUn3FeR4phMqH1XfG6LPKJePog",
            "cargo_amount": 1000,
            "from_sector": [42, 35],
            "to_sector": [40, 30]
        }
    }
}
```

## Solana Program Requests

See `sage-based-sdk` for the Solana Program requests (and audit of usage).

### Client and Payer Keypair Initialization

```
// bots/actix-minebot/src/main.rs
// bots > actix-minebot > src > main.rs > main
```

### SageBased Actor has "Ownership" of Client and Payer Keypair

```
// bots/actix-minebot/src/actors/sage/mod.rs
// bots > actix-minebot > src > actors > sage > mod.rs > SageBased
```

### Program Protection

```
// program-sdks/sage-based-sdk/src/lib.rs
// program-sdks > sage-based-sdk > src > lib.rs > {} impl SageBasedGameHandler > send_transaction

// protection against sending transactions to a "program" that is not the authorized Sage program
assert_eq!(program.id(), program::SAGE_ID, "invalid program id");
```

### Compute Budget/Priority Fee

The `MICRO_LAMPORTS` for the Compute Budget's `set_compute_unit_price` is defined here:

```
// program-sdks/sage-based-sdk/src/lib.rs
// program-sdks > sage-based-sdk > src > lib.rs > MICRO_LAMPORTS
```
